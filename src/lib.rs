mod macros;
mod map;

#[macro_export]
macro_rules! vector {
    () => {
        Vec::new()
    };
    ($($elem:expr),+ $(,)?) => {{
        let mut vec = Vec::with_capacity(count!(@COUNT; $($elem),*));
        $(vec.push($elem);)+
        vec
    }};
    ($elem:expr; $cap:expr) => {{
        let count = $cap;
        let mut vec = Vec::with_capacity(count);
        vec.resize(count, $elem);
        vec
    }}
}

#[macro_export]
macro_rules! count {
    (@COUNT; $($elem:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $elem]),*])
    };
    (@SUBST; $elem:expr) => {
        ()
    }
}

#[test]
fn empty_vector() {
    let vector: Vec<u32> = vector![];
    assert!(vector.is_empty());
}

#[test]
fn single_element() {
    let vector = vector!(24);
    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0], 24);
}

#[test]
fn multiple_element() {
    let vector = vector![24, 25];
    assert_eq!(vector.len(), 2);
    assert_eq!(vector[0], 24);
    assert_eq!(vector[1], 25);
}

#[test]
fn trailing_comma() {
    let vector = vector![24, 25,];
    assert_eq!(vector.len(), 2);
    assert_eq!(vector[0], 24);
    assert_eq!(vector[1], 25);
}

#[test]
fn cloning() {
    let vector = vector![24; 4];
    assert_eq!(vector.len(), 4);
    assert_eq!(vector[0], 24);
    assert_eq!(vector[1], 24);
    assert_eq!(vector[2], 24);
    assert_eq!(vector[3], 24);
}

#[test]
fn cloning_literal() {
    let mut y = Some(42);
    let vector = vector!(y.take().unwrap(); 4);
    assert_eq!(vector.len(), 4);
}

/// ```compile_fail
/// let x: Vec<u32> = vecmacro::vector![42, "foo"];
/// ```
#[allow(unused)]
struct CompileFailTest;
