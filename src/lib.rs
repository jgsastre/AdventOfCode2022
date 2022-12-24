use anyhow::Result;
use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

/*
struct Foo {
    bar: usize,
    baz: String,
}

struct Container {
    cell: Cell<Foo>
}


#[cfg(test)]
mod tests{
    
    use super::*;

    #[test]
    fn test() {
        
        let _container = Container{ cell: Cell::new(Foo{ bar: 1, baz: "foo".to_string() }) };

        let inner_value = _container.cell.get();




    }

}
*/



