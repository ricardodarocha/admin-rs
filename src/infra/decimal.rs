use serde::Deserialize;

pub fn decimal<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = s.replace(',', "."); // Substitui vírgula por ponto
    s.parse::<f32>().map_err(serde::de::Error::custom)
}