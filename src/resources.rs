use macroquad::texture::Texture2D;

pub struct Resources {
    pub uc_sign: Texture2D,
    pub rail_empty: Texture2D,
    pub rail_all: Texture2D,
    pub rail_h: Texture2D,
    pub rail_v: Texture2D,
    pub rail_ld: Texture2D,
    pub rail_lu: Texture2D,
    pub rail_rd: Texture2D,
    pub rail_ru: Texture2D,
    pub rail_lrd1: Texture2D,
    pub rail_lrd2: Texture2D,
    pub rail_lru1: Texture2D,
    pub rail_lru2: Texture2D,
}

impl Resources {
    pub fn load_textures() -> Resources {
        // load rail textures
        let uc_sign = include_bytes!("../assets/under_cons.png");
        let rail_empty = include_bytes!("../assets/empty.png");
        let rail_all = include_bytes!("../assets/rail_all.png");
        let rail_h = include_bytes!("../assets/rail_h.png");
        let rail_v = include_bytes!("../assets/rail_v.png");
        let rail_ld = include_bytes!("../assets/rail_ld.png");
        let rail_lu = include_bytes!("../assets/rail_lu.png");
        let rail_rd = include_bytes!("../assets/rail_rd.png");
        let rail_ru = include_bytes!("../assets/rail_ru.png");
        let rail_lrd1 = include_bytes!("../assets/rail_lrd1.png");
        let rail_lrd2 = include_bytes!("../assets/rail_lrd2.png");
        let rail_lru1 = include_bytes!("../assets/rail_lru1.png");
        let rail_lru2 = include_bytes!("../assets/rail_lru2.png");

        Resources {
            uc_sign: Texture2D::from_file_with_format(uc_sign, None),
            rail_empty: Texture2D::from_file_with_format(rail_empty, None),
            rail_all: Texture2D::from_file_with_format(rail_all, None),
            rail_h: Texture2D::from_file_with_format(rail_h, None),
            rail_v: Texture2D::from_file_with_format(rail_v, None),
            rail_ld: Texture2D::from_file_with_format(rail_ld, None),
            rail_lu: Texture2D::from_file_with_format(rail_lu, None),
            rail_rd: Texture2D::from_file_with_format(rail_rd, None),
            rail_ru: Texture2D::from_file_with_format(rail_ru, None),
            rail_lrd1: Texture2D::from_file_with_format(rail_lrd1, None),
            rail_lrd2: Texture2D::from_file_with_format(rail_lrd2, None),
            rail_lru1: Texture2D::from_file_with_format(rail_lru1, None),
            rail_lru2: Texture2D::from_file_with_format(rail_lru2, None),
        }
    }
}
