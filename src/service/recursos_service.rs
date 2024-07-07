use crate::models::recurso::Recurso;

pub fn get_recursos() -> Vec<Recurso> {
    vec![
        Recurso {
            id: 1,
            name: "Recurso 1".to_string(),
            description: "Descrição do recurso 1".to_string(),
        },
        Recurso {
            id: 2,
            name: "Recurso 2".to_string(),
            description: "Descrição do recurso 2".to_string(),
        },
    ]
}

pub fn create_recurso(recurso: Recurso) -> Result<Recurso, std::io::Error> {
    match true {
        true => Ok(recurso),
        false => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Erro ao criar recurso",
        )),
    }
}

pub(crate) fn modify_recurso(_id: u32, new_user: Recurso) -> Result<Recurso, std::io::Error> {
    match true {
        true => Ok(new_user),
        false => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Erro ao modificar recurso",
        )),
    }
}

pub(crate) fn delete_recurso(id: u32) -> Result<(), std::io::Error> {
    match id == 1 {
        true => Ok(()),
        false => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Erro ao deletar recurso",
        )),
    }
}
