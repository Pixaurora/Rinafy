use color_art::Color;
use imgref::ImgVec;

pub trait RinafyConvert<T> {
    fn convert(self) -> T;
}

impl RinafyConvert<image::Rgba<u8>> for Color {
    fn convert(self) -> image::Rgba<u8> {
        let (r, g, b) = (self.red(), self.green(), self.blue());
        let alpha = (self.alpha() * 255.0) as u8;

        image::Rgba([r, g, b, alpha])
    }
}

impl RinafyConvert<Color> for image::Rgba<u8> {
    fn convert(self) -> Color {
        let [r, g, b, alpha] = self.0;
        let alpha = alpha as f64 / 255.0;

        Color::new(r, g, b, alpha)
    }
}

impl RinafyConvert<image::Rgba<u8>> for rgb::RGBA8 {
    fn convert(self) -> image::Rgba<u8> {
        image::Rgba([self.r, self.g, self.b, self.a])
    }
}

impl RinafyConvert<rgb::RGBA8> for image::Rgba<u8> {
    fn convert(self) -> rgb::RGBA8 {
        let [r, g, b, a] = self.0;
        rgb::RGBA8 { r, g, b, a }
    }
}

impl RinafyConvert<ImgVec<rgb::RGBA8>> for image::DynamicImage {
    fn convert(self) -> ImgVec<rgb::RGBA8> {
        let width = self.width() as usize;
        let height = self.height() as usize;

        ImgVec::new(self.into_rgba8().pixels().collect(), width, height)
            .map_buf(|pixels| pixels.iter().map(|pixel| pixel.convert()).collect())
    }
}
