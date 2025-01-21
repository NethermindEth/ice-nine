#[macro_export]
macro_rules! name {
    ($($item:expr),* $(,)?) => {{
        let components = vec![$($item),*];
        $crate::names::Pqn::from_iter(components)
    }};
}

#[macro_export]
macro_rules! partial {
    ($text:expr) => {{
        let components = $crate::const_str::split!($text, '.');
        $crate::names::Pqn::from_iter(components)
    }};
}

#[macro_export]
macro_rules! full {
    ($text:expr) => {{
        let components = $crate::const_str::split!($text, '.');
        $crate::names::Fqn::from_iter(components)
    }};
}

#[cfg(test)]
mod tests {
    use crate::names::*;
    use anyhow::Result;

    #[test]
    fn test_name_macro() -> Result<()> {
        let expected = "one.two".parse()?;
        let name = Fqn::from(name!("one", "two"));
        assert_eq!(name, expected);
        Ok(())
    }

    #[test]
    fn test_partial_macro() -> Result<()> {
        let expected = "one.two".parse()?;
        let name = Fqn::from(partial!("one.two"));
        assert_eq!(name, expected);
        Ok(())
    }
}
