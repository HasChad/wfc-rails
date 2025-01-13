use macroquad::texture::Texture2D;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
#[exclude = "icons/"]
struct Asset;

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
        let uc_sign = &Asset::get("under_cons.png").unwrap().data.to_vec();
        let rail_empty = &Asset::get("empty.png").unwrap().data.to_vec();
        let rail_all = &Asset::get("rail_all.png").unwrap().data.to_vec();
        let rail_h = &Asset::get("rail_h.png").unwrap().data.to_vec();
        let rail_v = &Asset::get("rail_v.png").unwrap().data.to_vec();
        let rail_ld = &Asset::get("rail_ld.png").unwrap().data.to_vec();
        let rail_lu = &Asset::get("rail_lu.png").unwrap().data.to_vec();
        let rail_rd = &Asset::get("rail_rd.png").unwrap().data.to_vec();
        let rail_ru = &Asset::get("rail_ru.png").unwrap().data.to_vec();
        let rail_lrd1 = &Asset::get("rail_lrd1.png").unwrap().data.to_vec();
        let rail_lrd2 = &Asset::get("rail_lrd2.png").unwrap().data.to_vec();
        let rail_lru1 = &Asset::get("rail_lru1.png").unwrap().data.to_vec();
        let rail_lru2 = &Asset::get("rail_lru2.png").unwrap().data.to_vec();

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
