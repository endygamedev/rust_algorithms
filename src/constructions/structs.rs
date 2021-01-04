/*  Структура Range

    Аргументы:
        * start - начало итератора
        * stop - конец итератора
        * step - шар итератора
*/

pub struct Range(isize, isize, isize);

impl Range {
    pub fn new(start: isize, stop: isize, step: isize) -> Range {
        Range(start, stop, step)
    }
}

impl Iterator for Range {
    type Item = isize;

    #[inline]
    fn next(&mut self) -> Option<isize> {
        if self.0 < self.1 {
            let v: isize = self.0;
            self.0 = v + self.2;
            Some(v)
        } else { None }
    }
}
