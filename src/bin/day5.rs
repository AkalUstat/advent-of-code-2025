use aoc2025::akal_reader::read_lines;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct AkalRange {
    start: usize,
    end: usize,
}

impl AkalRange {
    fn update_bounds(self, new_range: &Self) -> Self {
        Self {
            start: self
                .start
                .min(new_range.start),
            end: self
                .end
                .max(new_range.end),
        }
    }

    fn create_range(&self) -> RangeInclusive<usize> {
        (self.start)..=(self.end)
    }

    fn does_overlap(&self, other: &Self) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
}

fn main() {
    // get lines
    let lines = read_lines("../aoc-inputs-2025/day5-1.txt").expect("Cannot Read File");
    let lines_vec: Vec<String> = lines
        .map_while(Result::ok)
        .collect();

    // get index of the empty line
    if let Some(separator_indx) = lines_vec
        .iter()
        .position(|elem| elem == "")
    {
        // that denotes the divide between the ranges and the provided ids
        let (fresh_ranges, ids) = lines_vec.split_at(separator_indx);
        // get rid of the empty line that is the separator
        let available_ids: Vec<usize> = ids
            .iter()
            // remove the separator empty line
            .filter(|el| !el.is_empty())
            // convert to number
            .filter_map(|str| {
                str.parse::<usize>()
                    .ok()
            })
            .collect();

        // convert bounds to the custom Range
        let mut bounds: Vec<AkalRange> = fresh_ranges
            .iter()
            .filter_map(|range| {
                // split the string
                let mut parts = range
                    .split("-")
                    .filter_map(|s| {
                        s.parse::<usize>()
                            .ok()
                    });

                // create new custom Range
                Some(AkalRange {
                    start: parts.next()?,
                    end: parts.next()?,
                })
            })
            .collect();

        // sort the bounds by the start value, so that we only need to compare to the previous
        // bounds for overlap
        bounds.sort_by_key(|range| range.start);

        // now, combine the overlapping ranges
        let ranges: Vec<RangeInclusive<usize>> = bounds
            // mutable iterator
            .into_iter()
            // use fold (to create a new vector)
            .fold(Vec::new(), |mut acc, x| {
                // if the acc is empty, just put in the range
                if acc.is_empty() {
                    acc.push(x);
                // get the last value
                } else if let Some(last_val) = acc
                    .last()
                    .cloned()
                {
                    // if there's overlap between current and last, combine and
                    // add the new range
                    if x.does_overlap(&last_val) {
                        acc.pop();
                        let new = x.update_bounds(&last_val);
                        acc.push(new);
                    } else {
                        // otherwise, push the current value
                        acc.push(x);
                    }
                }

                acc
            })
            .iter()
            // now convert to normal rust ranges
            .map(|akal_range| akal_range.create_range())
            .collect();

        // part 1
        let sum = available_ids
            // iterate over the given IDs
            .iter()
            // get only the IDs which are contained within a range.
            .filter(|x| {
                ranges
                    .iter()
                    .any(|range| range.contains(x))
            })
            // count
            .count();

        // part 2: just the total number of ingredients contained within a range
        let sum2 = ranges
            .iter()
            .map(|x| {
                // get the count of numbers contained within a range
                x.clone()
                    .count()
            })
            // sum the numbers of all ranges
            .sum::<usize>();

        println!("{} {}", sum, sum2);
    }
}
