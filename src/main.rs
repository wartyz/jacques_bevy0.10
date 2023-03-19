// https://www.youtube.com/watch?v=TQt-v_bFdao

use bevy::prelude::*;
use crate::Trabajo::{Abogado, Doctor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(print_names)
            .add_system(gente_con_trabajo)
            .add_system(gente_sin_trabajo)
            .add_system(gente_y_su_trabajo);
    }
}

pub fn setup(mut commands: Commands) {
    println!("hola");
    // Si es más de un Component, hay que ponerlos en una tupla
    commands.spawn(
        (
            Empleo {
                trabajo: Doctor,
            },
            Person {
                name: "Luis".to_string(),
            })
    );

    // No tiene Component "Empleado"
    commands.spawn(Person {
        name: "Pedro".to_string(),
    });

    commands.spawn(
        (
            Person {
                name: "Jose".to_string(),
            },
            Empleo {
                trabajo: Abogado,
            })
    );
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        dbg!(&person.name);
    }
}

pub fn gente_con_trabajo(
    person_query: Query<&Person, With<Empleo>>,
) {
    for person in person_query.iter() {
        println!("\nTienen trabajo -> {}", person.name)
    }
}

pub fn gente_sin_trabajo(
    person_query: Query<&Person, Without<Empleo>>,
) {
    for person in person_query.iter() {
        println!("\nNo Tienen trabajo -> {}", person.name)
    }
}

// Si hacemos una query a más de un Component, hay que ponerlos en una tupla
pub fn gente_y_su_trabajo(
    person_query: Query<(&Person, &Empleo)>
) {
    for (person, empleo) in person_query.iter() {
        println!("\nLas personas y su trabajo son trabajo -> {:?} y {:?}", person.name, empleo.trabajo)
    }
}

#[derive(Component, Debug)]
pub struct Person {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct Empleo {
    pub trabajo: Trabajo,
}

#[derive(Debug)]
pub enum Trabajo {
    Doctor,
    Oficinista,
    Abogado,
}
