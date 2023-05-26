use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Lezione {
    pub title: String,
    pub docente: String,
    pub time: String,
    pub teams: String,
    pub cfu: u8,
    pub val_crediti: u8,
    pub aule: Vec<LezioneAule>,
}

#[derive(Deserialize, Debug)]
pub struct LezioneAule {
    pub des_indirizzo: String,
    pub des_piano: String,
    pub des_edificio: String,
    pub des_ubicazione: String,
    pub des_risorsa: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Impegno {
    #[serde(rename = "dataInizio")]
    pub data_inizio: String,
    #[serde(rename = "dataFine")]
    pub data_fine: String,
    pub indisponibilita: bool,
    #[serde(rename = "indisponibilitaTotale")]
    pub indisponibilita_totale: bool,
    pub stato: String,
    pub nome: String,
    pub aule: Vec<Aula>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Aula {
    pub id: String,
    pub capienza: Option<i32>,
    #[serde(rename = "metriQuadri")]
    pub metri_quadri: f32,
    #[serde(rename = "numeroPostazioni")]
    pub numero_postazioni: Option<i32>,
    #[serde(rename = "tipoAulaId")]
    pub tipo_aula_id: String,
    pub descrizione: String,
    pub codice: String,
    pub abilitato: bool,
    #[serde(rename = "divisoreCapienza")]
    pub divisore_capienza: Option<i32>,
    pub piano: Piano,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Piano {
    pub codice: String,
    pub descrizione: String,
}
