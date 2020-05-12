use proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro]
pub fn build_test_renderer(input: TokenStream) -> TokenStream {
    let renderer_name = format_ident!("{}", input.to_string());

    let exp = quote! {
        let mut character_sizes = std::collections::HashMap::new();
        let mut canvas = CanvasMock::new();
        let config = build_config();
        let mut surface =
            sdl2::surface::Surface::new(512, 512, sdl2::pixels::PixelFormatEnum::RGB24).unwrap();
        let mut surface_canvas = sdl2::render::Canvas::from_surface(surface).unwrap();
        let mut texture_creator = surface_canvas.texture_creator();
        let mut texture_manager = crate::renderer::managers::TextureManager::new(&texture_creator);
        let mut ttf_context = sdl2::ttf::Sdl2TtfContext {};
        let mut #renderer_name = SimpleRendererMock {
            config: config.clone(),
            ttf: ttf_context,
            character_sizes,
            texture_manager,
        };
    };
    exp.into()
}
