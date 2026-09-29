use super::*;
fn document(text: &str) -> EditorDocument {
    EditorDocument::open(text.into(), EditorLimits::default()).unwrap()
}

#[test]
fn unicode_edit_undo_redo_and_branching_preserve_text() {
    let mut editor = document("A界🙂Z");
    editor.replace(1..4, "é").unwrap();
    assert_eq!(editor.text(), "Aé🙂Z");
    assert!(editor.is_dirty());
    assert!(editor.undo().unwrap());
    assert_eq!(editor.text(), "A界🙂Z");
    assert!(!editor.is_dirty());
    assert!(editor.redo().unwrap());
    assert_eq!(editor.text(), "Aé🙂Z");
    editor.undo().unwrap();
    editor.replace(1..4, "B").unwrap();
    assert!(!editor.redo().unwrap());
    assert_eq!(editor.text(), "AB🙂Z");
}

#[test]
fn invalid_utf8_boundaries_ranges_and_oversized_edits_are_atomic() {
    let mut editor = document("界");
    assert_eq!(editor.replace(1..2, "x"), Err(EditorError::Utf8Boundary));
    assert_eq!(editor.replace(3..4, "x"), Err(EditorError::InvalidRange));
    assert_eq!(
        editor.replace(0..0, &"x".repeat(MAX_DOCUMENT_BYTES)),
        Err(EditorError::DocumentTooLarge)
    );
    assert_eq!(editor.text(), "界");
    assert_eq!(editor.revision(), 0);
    assert!(!editor.undo().unwrap());
}

#[test]
fn save_conflict_returns_no_candidate_and_preserves_dirty_buffer() {
    let mut editor = document("disk");
    editor.replace(0..4, "draft").unwrap();
    assert!(matches!(
        editor.prepare_save("externally changed"),
        Err(EditorError::SaveConflict)
    ));
    assert_eq!(editor.text(), "draft");
    assert!(editor.is_dirty());
    let saved = editor.prepare_save("disk").unwrap();
    assert_eq!(saved.text(), "draft");
    assert_eq!(saved.expected_disk(), "disk");
    editor.acknowledge_saved(saved).unwrap();
    assert!(!editor.is_dirty());
}

#[test]
fn acknowledged_earlier_save_keeps_later_edit_dirty() {
    let mut editor = document("a");
    editor.replace(0..1, "b").unwrap();
    let candidate = editor.prepare_save("a").unwrap();
    editor.replace(0..1, "c").unwrap();
    editor.acknowledge_saved(candidate).unwrap();
    assert_eq!(editor.text(), "c");
    assert!(editor.is_dirty());
    assert!(editor.prepare_save("b").is_ok());
    assert!(matches!(
        editor.prepare_save("a"),
        Err(EditorError::SaveConflict)
    ));
}

#[test]
fn history_limits_keep_recent_undo_and_never_exceed_budget() {
    let limits = EditorLimits {
        document_bytes: 10,
        history_bytes: 20,
        history_steps: 2,
    };
    let mut editor = EditorDocument::open("0000000000".into(), limits).unwrap();
    for text in ["1111111111", "2222222222", "3333333333"] {
        editor.replace(0..10, text).unwrap();
        assert!(editor.retained_history_bytes() <= 20);
    }
    assert!(editor.undo().unwrap());
    assert_eq!(editor.text(), "2222222222");
    assert!(editor.undo().unwrap());
    assert_eq!(editor.text(), "1111111111");
    assert!(!editor.undo().unwrap());
    assert!(editor.redo().unwrap());
    assert!(editor.redo().unwrap());
    assert_eq!(editor.text(), "3333333333");
    assert!(editor.retained_history_bytes() <= 20);
}

#[test]
fn unchanged_edit_leaves_history_and_revision_alone() {
    let mut editor = document("same");
    editor.replace(0..4, "same").unwrap();
    assert_eq!(editor.revision(), 0);
    assert!(!editor.undo().unwrap());
}

#[test]
fn document_and_history_admission_limits_are_enforced() {
    assert!(matches!(
        EditorDocument::open("x".repeat(MAX_DOCUMENT_BYTES + 1), EditorLimits::default()),
        Err(EditorError::DocumentTooLarge)
    ));
    let invalid = EditorLimits {
        document_bytes: 10,
        history_bytes: 9,
        history_steps: 2,
    };
    assert!(matches!(
        EditorDocument::open(String::new(), invalid),
        Err(EditorError::InvalidLimits)
    ));
}

#[test]
fn same_baseline_save_candidate_cannot_acknowledge_another_document() {
    let mut first = document("a");
    let mut second = document("a");
    first.replace(0..1, "b").unwrap();
    second.replace(0..1, "c").unwrap();
    let second_candidate = second.prepare_save("a").unwrap();
    assert_eq!(
        first.acknowledge_saved(second_candidate.clone()),
        Err(EditorError::WrongDocument)
    );
    assert_eq!(first.text(), "b");
    assert!(first.is_dirty());
    assert!(first.prepare_save("a").is_ok());
    assert_eq!(first.revision(), 1);
    second.acknowledge_saved(second_candidate).unwrap();
    assert!(!second.is_dirty());
}
