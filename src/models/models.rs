use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Transacao {
    pub valor: u64,
    pub tipo: char,
    #[validate(length(min = 1, max = 10 ,message = "Quantidade de caracteres inválida"))]
    pub descricao: String
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Resposta {
    pub limite : u64,
    pub saldo : i64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Saldo {
    pub total: i64,
    #[serde(
    with = "time::serde::rfc3339",
    default = "OffsetDateTime::now_utc"
    )]
    pub data_transacao: OffsetDateTime,
    pub limite: i64

}

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename = "ultimas_transacoes")]
pub struct UltimasTransacoes {
    pub valor: u64,
    pub tipo: char,
    #[validate(length(min = 1, max = 10 ,message = "Quantidade de caracteres inválida"))]
    pub descricao: String,
    pub realizada_em: OffsetDateTime
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Cliente{
    pub id: u8,
    pub limite: u64,
    pub saldo_inicial: u8
}