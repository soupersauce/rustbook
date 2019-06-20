#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct Ref<'a, T: 'a>(&'a T);

struct StaticRef<T: 'static>(&'static T);
