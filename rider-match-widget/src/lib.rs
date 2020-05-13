use darling::{FromMeta, ToTokens};
use proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs};

#[derive(Debug, Clone)]
struct AssertBuilder {
    widget: String,
    widget_variable: Option<String>,
    contains: String,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    widget_dump_path: Option<String>,
    child_dump_path: Option<String>,
    background_from: Option<String>,
}

impl ToTokens for AssertBuilder {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let AssertBuilder {
            widget,
            contains,
            x,
            y,
            w,
            h,
            widget_dump_path,
            child_dump_path,
            widget_variable,
            background_from,
        } = self.clone();

        let var = format_ident!(
            "{}",
            widget_variable
                .as_ref()
                .cloned()
                .unwrap_or_else(|| "widget".to_string())
        );
        let widget_name = format_ident!("{}", widget);
        let child_name = format_ident!("{}", contains);
        let background = match background_from {
            Some(name) => {
                let ident = format_ident!("{}", name);
                quote! {
                    surface_canvas
                        .render_rect(size.clone(), widget. #ident .clone())
                        .unwrap();
                }
            }
            _ => quote! {},
        };
        let widget_dump_path = match widget_dump_path.as_ref() {
            Some(t) => {
                quote! { Some( #t ) }
            }
            None => quote! { None },
        };
        let child_dump_path = match child_dump_path.as_ref() {
            Some(t) => {
                quote! { Some( #t ) }
            }
            None => quote! { None },
        };

        let generated = quote! {
            let mut character_sizes = std::collections::HashMap::new();
            let config = build_config();
            let mut surface =
                sdl2::surface::Surface::new( #x as u32 + #w as u32 , #y as u32 + #h as u32, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap();
            let mut surface_canvas = sdl2::render::Canvas::from_surface(surface).unwrap();
            let mut texture_creator = surface_canvas.texture_creator();
            let mut texture_manager = crate::renderer::managers::TextureManager::new(&texture_creator);
            let mut ttf_context = sdl2::ttf::Sdl2TtfContext {};
            let mut renderer = SimpleRendererMock {
                config: config.clone(),
                ttf: ttf_context,
                character_sizes,
                texture_manager,
            };

            let mut #var = #widget_name ::new(config);
            #var .prepare_ui();
            let size = sdl2::rect::Rect::new(
                0i32,
                0i32,
                #w as u32 + #x  as u32,
                #h as u32 + #y  as u32,
            );
            let expected_pixels = {
                #background
                #var . #child_name .render(
                    &mut surface_canvas,
                    &mut renderer,
                    &RenderContext::ParentPosition( size.top_left() .offset( #x , #y ) ),
                );
                surface_canvas
                    .read_pixels(sdl2::rect::Rect::new( #x as i32, #y as i32 , #w as u32 , #h as u32 ), PixelFormatEnum::RGBA8888)
                    .unwrap()
            };
            if let Some(child_dump_path) = #child_dump_path {
                surface_canvas.dump_ui(child_dump_path);
            }

            #var .render(&mut surface_canvas, &mut renderer, &RenderContext::Nothing);
            if let Some(widget_dump_path) = #widget_dump_path {
                surface_canvas.dump_ui(widget_dump_path);
            }
            let result_pixels = surface_canvas
                .read_pixels(sdl2::rect::Rect::new( #x as i32, #y as i32, #w as u32, #h as u32 ), PixelFormatEnum::RGBA8888)
                .unwrap();

            if expected_pixels != result_pixels {
                match ( #child_dump_path, #widget_dump_path ) {
                    (Some(c), Some(w)) => {
                        assert_eq!(c, w);
                    }
                    _ => {
                        assert_eq!(result_pixels, expected_pixels);
                    }
                }
            }
        };

        tokens.extend(generated);
    }
}

#[derive(Debug, Clone, FromMeta)]
struct AssertOptions {
    fn_name: String,
    widget: String,
    contains: String,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    widget_dump_path: Option<String>,
    child_dump_path: Option<String>,
    widget_variable: Option<String>,
    background_from: Option<String>,
}

#[proc_macro_attribute]
pub fn match_widgets(args: TokenStream, _input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    // let _input = parse_macro_input!(input as ItemFn);

    let opts = match AssertOptions::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let builder = AssertBuilder {
        widget: opts.widget,
        widget_variable: opts.widget_variable,
        contains: opts.contains,
        x: opts.x,
        y: opts.y,
        w: opts.w,
        h: opts.h,
        widget_dump_path: opts.widget_dump_path,
        child_dump_path: opts.child_dump_path,
        background_from: opts.background_from,
    };

    let name = format_ident!("{}", opts.fn_name);

    let output = quote! { fn #name () { #builder } };
    output.into()
}
