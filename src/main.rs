use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    style,
    terminal::{self, size},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::io::{stdout, Stdout, Write};
use std::time;

fn main() {
    let (cols, rows) = size().unwrap();

    let mut stdout = stdout();

    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    loop {
        // Testing out this method because it's neater than just putting drawing code directly inside
        // of a regular sorting algorithm
        let data = create_data(rows, cols);
        let mut bubble_sort_stepper = BubbleSort::init(data);

        while !bubble_sort_stepper.has_completed() {
            bubble_sort_stepper.step();

            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
            queue_data_draw(&mut stdout, &bubble_sort_stepper.data, rows);
            stdout.flush().unwrap();

            if blocking_poll_for_cntrlc(time::Duration::from_millis(25)) {
                stdout.queue(cursor::MoveTo(0, rows)).unwrap();
                stdout.flush().unwrap();
                terminal::disable_raw_mode().unwrap();
                stdout.execute(cursor::Show).unwrap();
                return;
            }
            if blocking_poll_for_key(time::Duration::from_millis(25), KeyCode::Char('n')) {
                break;
            }
        }

        let data = create_data(rows, cols);
        let mut selection_sort_stepper = SelectionSort::init(data);

        while !selection_sort_stepper.has_completed() {
            selection_sort_stepper.step();

            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
            queue_data_draw(&mut stdout, &selection_sort_stepper.data, rows);
            stdout.flush().unwrap();

            if blocking_poll_for_cntrlc(time::Duration::from_millis(25)) {
                stdout.queue(cursor::MoveTo(0, rows)).unwrap();
                stdout.flush().unwrap();
                terminal::disable_raw_mode().unwrap();
                stdout.execute(cursor::Show).unwrap();
                return;
            }
            if blocking_poll_for_key(time::Duration::from_millis(25), KeyCode::Char('n')) {
                break;
            }
        }

        let data = create_data(rows, cols);
        let mut insertion_sort_stepper = InsertionSort::init(data);

        while !insertion_sort_stepper.has_completed() {
            insertion_sort_stepper.step();

            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
            queue_data_draw(&mut stdout, &insertion_sort_stepper.data, rows);
            stdout.flush().unwrap();

            if blocking_poll_for_cntrlc(time::Duration::from_millis(8)) {
                stdout.queue(cursor::MoveTo(0, rows)).unwrap();
                stdout.flush().unwrap();
                terminal::disable_raw_mode().unwrap();
                stdout.execute(cursor::Show).unwrap();
                return;
            }
            if blocking_poll_for_key(time::Duration::from_millis(8), KeyCode::Char('n')) {
                break;
            }
        }
    }
}

struct BubbleSort {
    data: Vec<u16>,
    swap_limit: usize,
}

impl BubbleSort {
    fn init(data: Vec<u16>) -> Self {
        BubbleSort {
            swap_limit: data.len() - 1,
            data,
        }
    }

    fn step(&mut self) {
        if self.swap_limit < self.data.len() {
            let mut swapped = false;
            for i in 0..self.swap_limit {
                if self.data[i] > self.data[i + 1] {
                    let t = self.data[i + 1];
                    self.data[i + 1] = self.data[i];
                    self.data[i] = t;
                    swapped = true;
                }
            }

            self.swap_limit -= 1;

            if !swapped {
                self.swap_limit = self.data.len() + 1;
            }
        }
    }

    fn has_completed(&self) -> bool {
        self.swap_limit > self.data.len()
    }
}

struct SelectionSort {
    data: Vec<u16>,
    selected: usize,
}

impl SelectionSort {
    fn init(data: Vec<u16>) -> Self {
        SelectionSort { data, selected: 0 }
    }

    fn step(&mut self) {
        if self.selected < self.data.len() {
            let mut index_of_min = self.selected;
            for index in self.selected..self.data.len() {
                if self.data[index_of_min] > self.data[index] {
                    index_of_min = index;
                }
            }
            let t = self.data[index_of_min];
            self.data[index_of_min] = self.data[self.selected];
            self.data[self.selected] = t;

            self.selected += 1;
        }
    }

    fn has_completed(&self) -> bool {
        self.selected == self.data.len() - 1
    }
}

struct InsertionSort {
    data: Vec<u16>,
    last_sorted: usize,
    current_index: usize,
}

impl InsertionSort {
    fn init(data: Vec<u16>) -> Self {
        InsertionSort {
            data,
            last_sorted: 1,
            current_index: 1,
        }
    }

    fn step(&mut self) {
        if self.last_sorted <= self.data.len() {
            if self._step() {
                self.last_sorted += 1;
                self.current_index = self.last_sorted - 1;
            }
        }
    }

    fn _step(&mut self) -> bool {
        if self.current_index > 0 {
            if self.data[self.current_index] < self.data[self.current_index - 1] {
                let t = self.data[self.current_index - 1];
                self.data[self.current_index - 1] = self.data[self.current_index];
                self.data[self.current_index] = t;
            } else {
                return true;
            }
            self.current_index -= 1;
            false
        } else {
            true
        }
    }

    fn has_completed(&self) -> bool {
        self.last_sorted > self.data.len()
    }
}

fn create_data(rows: u16, cols: u16) -> Vec<u16> {
    let mut vec = Vec::new();
    for _ in 0..cols {
        let rand_num = rand::thread_rng().gen_range(3..rows);
        vec.push(rand_num);
    }
    vec
}

fn queue_data_draw(stdout: &mut Stdout, data: &[u16], rows: u16) {
    for (index, bar_height) in data.iter().enumerate() {
        for dy in 1..*bar_height {
            stdout
                .queue(cursor::MoveTo(index as u16, rows - dy))
                .unwrap()
                .queue(style::Print("█"))
                .unwrap();
        }
    }

    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
}

fn blocking_poll_for_key(delay: time::Duration, key: KeyCode) -> bool {
    if poll(delay).unwrap() {
        return match read().unwrap() {
            Event::Key(event) => event.code == key,
            _ => false,
        };
    }
    false
}

fn blocking_poll_for_cntrlc(delay: time::Duration) -> bool {
    if poll(delay).unwrap() {
        return match read().unwrap() {
            Event::Key(event) => event.code == KeyCode::Char('c') && event.modifiers.bits() == 2,
            _ => false,
        };
    }
    false
}
