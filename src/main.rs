use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

// screen is fixed for now: 800 x 400
// Bridges:
// Bridge height:
// - on average 1/4 of screen height (100 pixels) 
// - max is averge + 50% (150 pixels)
// - min is average -50% (50 pixels)
// Bridge length:
// - on average 1/4 of screen width (200 pixels)
// - max is average + 50% (300 pixels)
// - min is average - 50% (100 pixels)

// total length of Bridges + spaces must be more than the screen width
// a new element is added when it becomes less

const SCREEN_HEIGHT: u32 = 400;
const SCREEN_WIDTH: u32 = 800;

/// Bridge is defined by its height and length
/// TO DO: a type to be added, to draw different bridges.
///        Needs 3 elements for a bridge type: left, right, middle.
///        Probably use a spritesheet for it.
/// Space is defined by its length only. It is the space between buildings.

// There are two types of Elements: Bridge and Space
enum Kind {
    Bridge,
    Space
}

struct Element {
    kind: Kind,
    height: u32,
    length: u32
}

impl Element {
    fn new(kind: &Kind) -> Element {
        match kind {
            Kind::Bridge => {
                let heights: [u32; 3] = [50,100,150];
                let lengths: [u32; 3] = [100,200,300];
                let mut rng = thread_rng();
                let length: usize = rng.gen_range(0..3);
                let height: usize = rng.gen_range(0..3);
                Element {
                    kind: Kind::Bridge,
                    height: heights[height],
                    length: lengths[length],
                }
            },
            Kind::Space => {
                let lengths: [u32; 3] = [100, 150, 200];
                let mut rng = thread_rng();
                let length = rng.gen_range(0..3);
                Element {
                    kind: Kind::Space,
                    height: 0,
                    length: lengths[length],
                }
            }
        }
    }
}

/// A path is an ordered Bridge and Space which total length is
/// more than the width of the screen. There is only one Path per game.
struct Path {
    set: Vec<Element>
}

impl Path {
    /// create  a new Path. Only one Path is needed per game.
    /// The Path is created with enough Element to fill the screen width
    fn new() -> Path {
        let mut set = Vec::<Element>::new();
        let mut total_length = 0;
        let mut toggle = Kind::Bridge;
        while total_length <= SCREEN_WIDTH {
            let element = Element::new(&toggle);
            total_length += element.length;
            set.push(element);
            match toggle {
                Kind::Space => {toggle = Kind::Bridge},
                Kind::Bridge=> {toggle = Kind::Space}
            }
        }
        Path {
            set
        }
    }

    /// add an Element to a Path. The new Element is added a the end of the
    /// LinkedList.
    /// Returns the number of Elements in the list if Ok, or an
    /// error message if Err
    fn add (&mut self)-> std::result::Result<i32,&'static str> {
        // What is the last Element in the list ?
        match &self.set[(self.set.len())-1].kind {
            Kind::Bridge => {
                // create a new Space
                let space = Element::new(&Kind::Space);
                self.set.push(space);
            },
            Kind::Space => {
                // create a new Bridge
                let bridge = Element::new(&Kind::Bridge);
                self.set.push(bridge);
            }
        }
        Ok(1)
    }

    fn drop(&mut self) -> std::result::Result<Element,&'static str> {
        let removed_element = self.set.remove(0);
        println!("Removed {}", removed_element);
        Ok(removed_element)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.kind {
            Kind::Bridge => {
                write!(f,"(Bridge, length = {}, height = {})",self.length,self.height).unwrap();
            },
            Kind::Space => {
                write!(f,"(Space, length = {}, height = {})",self.length,self.height).unwrap();
            }
        }
        Ok(())
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f,"{{").unwrap();
        for i in &self.set {
            println!("{i}");
        };
        write!(f,"}}").unwrap();
        Ok(())
    }
}
fn main() {
    let mut path = Path::new();
    println!("Initial path {}",path);
    for i in 1..10 {
        // remove first element
        path.drop().unwrap();
        // and add a new one at the end
        path.add().unwrap();
        println!("loop {i}{path}");
    }
}
