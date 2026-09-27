use bastion_core::console::{Command as C, Event as E, LineEditor, OutputQueue};
fn feed(editor: &mut LineEditor, bytes: &[u8]) -> E {
    bytes.iter().fold(E::None, |_, b| editor.feed(*b))
}
#[test]
fn editing_crlf_cancellation_and_commands() {
    let mut e = LineEditor::new();
    assert_eq!(feed(&mut e, b"hexx\x08\x7flp\r"), E::Submit(C::Help));
    assert_eq!(e.feed(b'\n'), E::None);
    assert_eq!(feed(&mut e, b"ps\n"), E::Submit(C::Processes));
    assert_eq!(feed(&mut e, b"garbage\x03"), E::Cancel);
    assert_eq!(feed(&mut e, b"unused\x15"), E::Clear(6));
    for (text, cmd) in [
        ("limits", C::Limits),
        ("net status", C::Network),
        ("uptime", C::Uptime),
        ("version", C::Version),
        ("?", C::Help),
        ("", C::Empty),
        ("unknown", C::Unknown),
    ] {
        feed(&mut e, text.as_bytes());
        assert_eq!(e.feed(b'\r'), E::Submit(cmd));
    }
}
#[test]
fn overflow_cannot_execute_a_truncated_command_or_be_undone_by_backspace() {
    let mut e = LineEditor::new();
    feed(&mut e, b"help");
    feed(&mut e, &[b' '; 60]);
    assert_eq!(e.feed(b'\n'), E::Submit(C::Help)); // exact capacity is legal
    feed(&mut e, b"help");
    feed(&mut e, &[b' '; 61]);
    feed(&mut e, &[8; 64]);
    assert_eq!(e.feed(b'\n'), E::Rejected);
    assert_eq!(feed(&mut e, b"help\n"), E::Submit(C::Help));
}
#[test]
fn invalid_bytes_and_uart_errors_reject_whole_lines() {
    for byte in [0, 9, 27, 128, 255] {
        let mut e = LineEditor::new();
        feed(&mut e, b"he");
        e.feed(byte);
        assert_eq!(feed(&mut e, b"lp\n"), E::Rejected);
        assert_eq!(feed(&mut e, b"ps\n"), E::Submit(C::Processes));
    }
    let mut e = LineEditor::new();
    feed(&mut e, b"help");
    e.reject();
    assert_eq!(e.feed(b'\n'), E::Rejected);
    feed(&mut e, &[b'x'; 65]);
    e.feed(21);
    assert_eq!(feed(&mut e, b"ps\n"), E::Submit(C::Processes));
}
#[test]
fn sustained_input_recovers_without_growing_storage() {
    let mut e = LineEditor::new();
    for byte in (0..65536).map(|n| n as u8) {
        e.feed(byte);
    }
    e.feed(3);
    assert_eq!(feed(&mut e, b"help\n"), E::Submit(C::Help));
}
#[test]
fn output_backpressure_is_atomic_and_wraps_without_losing_bytes() {
    let mut q = OutputQueue::<4>::new();
    assert!(q.push(b"abc"));
    assert!(!q.push(b"\r\n"));
    assert_eq!(q.pop(), Some(b'a'));
    assert!(q.push(b"\r\n"));
    assert_eq!(
        [q.pop(), q.pop(), q.pop(), q.pop()],
        [Some(b'b'), Some(b'c'), Some(b'\r'), Some(b'\n')]
    );
    assert_eq!(q.pop(), None);
    let mut empty = OutputQueue::<0>::new();
    assert!(empty.push(b""));
    assert!(!empty.push(b"x"));
    assert_eq!(empty.pop(), None);
}
