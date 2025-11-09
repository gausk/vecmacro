/*
macro_rules! macvar {
  () => {
      let x = 31;
  }
}

fn foo() {
    macvar!();
    x+1
}

This does not compile as the identifiers in macro world
are just completely distinct from outside world.
*/

/*
Macro are not allowed to access things outside their scope.

macro_rules! mac {
    () => {
        x +=1;
    };
}

fn foo() {
    let mut x = 1;
    mac!()
}
*/

macro_rules! working {
    ($x:ident) => {
        $x += 1;
    };
}

#[allow(unused)]
fn working_foo() -> usize {
    let mut x = 1;
    working!(x);
    x
}

#[test]
fn test_working_foo() {
    assert_eq!(working_foo(), 2);
}

#[allow(unused)]
trait MaxValue {
    fn max_val() -> Self;
}

macro_rules! max_value {
    ($t:ty) => {
        impl MaxValue for $t {
            fn max_val() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_value!(u8);
max_value!(u16);
max_value!(u32);
max_value!(u64);

#[test]
fn test_max_value() {
    assert_eq!(u8::max_val(), u8::MAX);
}
