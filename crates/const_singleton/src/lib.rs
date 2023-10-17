use std::cell::OnceCell;

#[derive(Debug, Clone)]
pub struct ConstSingleton<T, F>
where
    F: Fn() -> T,
{
    f: F,
    cell: OnceCell<T>,
}

impl<T, F: Fn() -> T> ConstSingleton<T, F> {
    pub fn get(&self) -> &T {
        self.cell.get().unwrap_or_else(|| {
            self.cell
                .set((self.f)())
                .unwrap_or_else(|_| panic!("singleton super fail"));
            let result = self.cell.get().unwrap();
            result
        })
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    const CS: ConstSingleton<i32, fn() -> i32> = ConstSingleton {
        cell: OnceCell::new(),
        f: || 42i32,
    };

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(CS.get(), &42);
        assert_eq!(CS.get(), &42);
    }
}
