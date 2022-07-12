use image::Rgba;

pub struct ColorData {
    kind: ColorKind,
    name: &'static str,
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Copy)]
pub enum ColorKind {
    Entity,
    Tile,
}

impl ColorData {
    const fn new(kind: ColorKind, name: &'static str, r: u8, g: u8, b: u8) -> Self {
        Self { kind, name, r, g, b }
    }
}

pub const COLOR_DATA: &[ColorData] = &[
    ColorData::new(ColorKind::Entity, "underground-belt", 115, 89, 0),
    ColorData::new(ColorKind::Entity, "transport-belt", 206, 162, 66),
    ColorData::new(ColorKind::Entity, "pipe", 66, 129, 164),
    ColorData::new(ColorKind::Entity, "iron-chest", 0, 97, 148),
    ColorData::new(ColorKind::Entity, "stone-wall", 206, 219, 206),
    ColorData::new(ColorKind::Entity, "heat-pipe", 58, 129, 173),
    ColorData::new(ColorKind::Entity, "gate", 123, 125, 123),
    ColorData::new(ColorKind::Tile, "stone-path", 82, 81, 74),
    ColorData::new(ColorKind::Tile, "concrete", 58, 61, 58),
    ColorData::new(ColorKind::Tile, "hazard-concrete-left", 181, 142, 33),
    ColorData::new(ColorKind::Tile, "refined-concrete", 49, 49, 41),
    ColorData::new(ColorKind::Tile, "refined-hazard-concrete-left", 115, 93, 25),
];

pub struct GlobalColorMap;

impl image::imageops::ColorMap for GlobalColorMap {
    type Color = Rgba<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        if color.0.get(3).unwrap() < &16 {
            return usize::MAX;
        }

        let mut closest_difference = u32::MAX;
        let mut closest_index = 0;
        for (index, data) in COLOR_DATA.iter().enumerate() {
            let difference = (*color.0.get(0).unwrap() as i32).abs_diff(data.r as i32)
                + (*color.0.get(1).unwrap() as i32).abs_diff(data.g as i32)
                + (*color.0.get(2).unwrap() as i32).abs_diff(data.b as i32);

            if difference < closest_difference {
                closest_index = index;
                closest_difference = difference;
            }
        }

        closest_index
    }

    fn map_color(&self, color: &mut Self::Color) {
        let index = self.index_of(color);
        if index == usize::MAX {
            color.0 = [0, 0, 0, 0];
            return;
        }
        let data = COLOR_DATA.get(index).unwrap();
        color.0 = [data.r, data.g, data.b, 0xff];
    }
}

pub fn nearest_color(source: (u8, u8, u8)) -> (ColorKind, &'static str) {
    let mut closest_name = "";
    let mut closest_kind = ColorKind::Entity;
    let mut closest_difference = u32::MAX;

    for color in COLOR_DATA {
        let difference = (source.0 as i32).abs_diff(color.r as i32)
            + (source.1 as i32).abs_diff(color.g as i32)
            + (source.2 as i32).abs_diff(color.b as i32);

        if difference < closest_difference {
            closest_name = color.name;
            closest_kind = color.kind;
            closest_difference = difference;
        }
    }

    (closest_kind, closest_name)
}
