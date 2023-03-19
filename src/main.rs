use bevy::prelude::*;
use crate::Trabajo::Doctor;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(print_names)
        .run();
}

pub fn setup(mut commands: Commands) {
    println!("hola");
    commands.spawn(
        (
            Empleo {
                trabajo: Doctor,
            },
            Person {
                name: "Luis".to_string(),
            })
    );

    commands.spawn(Person {
        name: "Pedro".to_string(),
    });

    commands.spawn(Person {
        name: "Jose".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        dbg!(&person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Empleo {
    pub trabajo: Trabajo,
}

#[derive(Debug)]
pub enum Trabajo {
    Doctor,
    Oficinista,
    Abogado,
}
