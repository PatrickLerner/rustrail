use osmpbfreader::{OsmObj, Way};

pub fn is_rail(obj: &Way) -> bool {
    obj.tags.contains("railway", "rail")
}

pub fn is_wood(obj: &Way) -> bool {
    obj.tags.contains("natural", "wood")
}

pub fn is_water(obj: &Way) -> bool {
    obj.tags.contains_key("water")
}

pub fn is_building(obj: &Way) -> bool {
    obj.tags.contains_key("building")
}

pub fn is_roof_building(obj: &Way) -> bool {
    obj.tags.contains("building", "roof")
}

pub fn is_industrial_building(obj: &Way) -> bool {
    obj.tags.contains("building", "industrial")
}

pub fn is_office_building(obj: &Way) -> bool {
    obj.tags.contains("building", "office")
}

pub fn is_commercial_building(obj: &Way) -> bool {
    obj.tags.contains("building", "commercial")
}

pub fn is_railway_platform(obj: &Way) -> bool {
    obj.tags.contains("railway", "platform")
}

pub fn is_relevant_object(obj: &OsmObj) -> bool {
    if let OsmObj::Way(obj) = obj {
        return is_rail(obj)
            || is_building(obj)
            || is_railway_platform(obj)
            || is_wood(obj)
            || is_water(obj);
    }

    false
}
