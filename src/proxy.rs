// 定义共享接口
trait Image {
    fn display(&self);
}

// 原始对象
struct RealImage {
    filename: String,
}

impl RealImage {
    fn new(filename: String) -> RealImage {
        RealImage { filename }
    }

    fn load_image(&self) {
        println!("Loading image: {}", self.filename);
    }
}

impl Image for RealImage {
    fn display(&self) {
        self.load_image();
        println!("Displaying image: {}", self.filename);
    }
}

// 代理对象
struct ProxyImage {
    real_image: Option<RealImage>,
    filename: String,
}

impl ProxyImage {
    fn new(filename: String) -> ProxyImage {
        ProxyImage {
            real_image: None,
            filename,
        }
    }

    fn load_image(&mut self) {
        if self.real_image.is_none() {
            self.real_image = Some(RealImage::new(self.filename.clone()));
        }
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        self.load_image();

        if let Some(real_image) = &self.real_image {
            real_image.display();
        }
    }
}

fn main() {
    let image1 = ProxyImage::new("image1.jpg".to_string());
    image1.display();

    let image2 = ProxyImage::new("image2.jpg".to_string());
    image2.display();
}
