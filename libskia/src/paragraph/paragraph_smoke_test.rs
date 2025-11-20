use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle};
use skia_safe::FontMgr;
use std::collections::HashMap;
use std::hint::black_box;
use std::num::NonZero;
use std::{panic, ptr};

use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[unsafe(no_mangle)]
pub extern "C" fn paragraph_smoke(style: *mut ValueBox<ParagraphStyle>) {
    style.with_ref_ok(|style| smoke_test_paragraph(style)).log();
}

const BUF_SIZE: usize = 4096;

#[derive(Debug, Clone)]
#[repr(C)]
struct SmokeTestState {
    start: u64,
    end: u64,
    glyph_index: usize,
    is_left: bool,
    coordinate: f32,
    done: bool,
}

fn smoke_test_paragraph(paragraph_style: &ParagraphStyle) {
    let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let max_len = 5;
    let threads = std::thread::available_parallelism()
        .unwrap_or(NonZero::new(1).unwrap())
        .get();

    let total = StringIterator::total_count(alphabet.len(), max_len);

    let style = paragraph_style.clone();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), None);

    let mut b = ParagraphBuilder::new(&style, font_collection.clone());
    b.add_text("haba");
    let mut p = b.build();
    p.layout(9999.0);

    // PID → pointer to shared memory region
    let mut regions: HashMap<i32, *mut u8> = HashMap::new();

    // -------- Parent waits for all children --------
    let mut remaining = 0;

    for i in 0..threads {
        let start = total * i as u64 / threads as u64;
        let end = total * (i as u64 + 1) / threads as u64;

        remaining += 1;
        fork_smoke_test(
            alphabet.to_vec(),
            max_len,
            &style,
            &mut font_collection,
            &mut regions,
            SmokeTestState {
                start,
                end,
                glyph_index: 0,
                is_left: true,
                coordinate: 0.0,
                done: false,
            },
        );
    }

    let mut crashes = vec![];

    while remaining > 0 {
        unsafe {
            let mut status: libc::c_int = 0;

            let pid = libc::waitpid(-1, &mut status, 0);
            // -1 means "wait for ANY child"

            if pid <= 0 {
                panic!("waitpid failed");
            }

            let region = regions.remove(&pid).unwrap();

            // Read the header
            let state = (&*(region as *const SmokeTestState)).clone();

            libc::munmap(region as *mut libc::c_void, BUF_SIZE);

            if libc::WIFEXITED(status) {
                let code = libc::WEXITSTATUS(status);
                println!("Child {pid} exited with code {code}");
            } else if libc::WIFSIGNALED(status) {
                if !state.done {
                    let mut iter = StringIterator::new(alphabet, max_len, state.start, state.end);
                    let bytes = iter.next().unwrap();

                    crashes.push((String::from_utf8_unchecked(bytes), state.coordinate));

                    let mut next_state = state.clone();
                    if next_state.is_left {
                        next_state.is_left = false;
                    } else {
                        next_state.is_left = true;
                        next_state.glyph_index += 1;
                    }

                    remaining += 1;
                    fork_smoke_test(
                        alphabet.to_vec(),
                        max_len,
                        &style,
                        &mut font_collection,
                        &mut regions,
                        next_state,
                    );
                }
            }

            remaining -= 1;
        }
    }

    println!("All children completed.");
    println!(
        "Tested {} strings, detected {} crashes. {}% crash rate",
        total,
        crashes.len(),
        ((crashes.len() as f64 / total as f64) * 100.0)
    );
}

fn fork_smoke_test(
    alphabet: Vec<u8>,
    max_len: usize,
    style: &ParagraphStyle,
    font_collection: &mut FontCollection,
    regions: &mut HashMap<i32, *mut u8>,
    state: SmokeTestState,
) {
    unsafe {
        // -------- mmap shared region ----------
        let ptr = libc::mmap(
            ptr::null_mut(),
            BUF_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        if ptr == libc::MAP_FAILED {
            panic!("mmap failed");
        }

        match libc::fork() {
            -1 => panic!("fork failed"),
            0 => {
                if state.start == 0 {
                    //std::thread::sleep(std::time::Duration::from_secs(30));
                }

                let region = ptr as *mut u8;
                ptr::copy_nonoverlapping(
                    &state.clone() as *const SmokeTestState as *const u8,
                    region,
                    size_of::<SmokeTestState>(),
                );
                let current_state = &mut *(ptr as *mut SmokeTestState);

                let iter = StringIterator::new(&alphabet, max_len, state.start, state.end);
                for bytes in iter {
                    let string = unsafe { String::from_utf8_unchecked(bytes) };
                    smoke_test_string(&string, &font_collection, &style, current_state);
                    current_state.start += 1;
                }
                current_state.done = true;
                libc::exit(0);
            }
            pid => {
                regions.insert(pid, ptr as *mut u8);
            }
        }
    }
}

fn smoke_test_string(
    string: &str,
    font_collection: &FontCollection,
    paragraph_style: &ParagraphStyle,
    state: &mut SmokeTestState,
) {
    let mut builder = ParagraphBuilder::new(paragraph_style, font_collection.clone());
    builder.add_text(string);
    let mut paragraph = builder.build();
    paragraph.layout(999999.0);

    for i in state.glyph_index..string.len() {
        if let Some(glyph) = paragraph.get_glyph_info_at_utf16_offset(i) {
            state.glyph_index = i;
            if state.is_left {
                state.coordinate = glyph.grapheme_layout_bounds.left;
                let position = paragraph
                    .get_glyph_position_at_coordinate((glyph.grapheme_layout_bounds.left, 14.0));
                black_box(position);
            }

            state.is_left = false;
            state.coordinate = glyph.grapheme_layout_bounds.right;
            let position = paragraph
                .get_glyph_position_at_coordinate((glyph.grapheme_layout_bounds.right, 14.0));
            black_box(position);

            state.is_left = true;
        }
    }
    state.glyph_index = 0;
}

pub struct StringIterator<'a> {
    alphabet: &'a [u8],
    max_len: usize,
    current_id: u64,
    end_id: u64,
}

impl<'a> StringIterator<'a> {
    pub fn new(alphabet: &'a [u8], max_len: usize, start: u64, end: u64) -> Self {
        Self {
            alphabet,
            max_len,
            current_id: start,
            end_id: end,
        }
    }

    // Precompute total count of all strings < max_len
    pub fn total_count(alphabet_len: usize, max_len: usize) -> u64 {
        let mut total = 1u64; // empty string
        let mut pow = 1u64;
        for _ in 1..max_len {
            pow *= alphabet_len as u64;
            total += pow;
        }
        total
    }

    // Convert numeric ID into a string
    fn id_to_string(&self, mut id: u64) -> Vec<u8> {
        let k = self.alphabet.len() as u64;

        // find length bucket
        let mut length = 0usize;
        let mut count = 1u64;
        let mut consumed = 0u64;

        // length 0
        if id == 0 {
            return vec![];
        }
        id -= 1; // skip empty string
        consumed += 1;

        for l in 1..self.max_len {
            count *= k;
            if id < count {
                length = l;
                break;
            }
            id -= count;
            consumed += count;
        }

        // convert id to base-k with `length` digits
        let mut s = vec![0u8; length];
        for i in 0..length {
            let idx = (id % k) as usize;
            s[length - 1 - i] = self.alphabet[idx];
            id /= k;
        }
        s
    }
}

impl<'a> Iterator for StringIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id >= self.end_id {
            return None;
        }
        let s = self.id_to_string(self.current_id);
        self.current_id += 1;
        Some(s)
    }
}
