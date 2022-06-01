struct Config {
    flash_on_f: bool,
    revert_patch: bool,
    preferred_item_slots: (),
    small_items: (),
    backend: BackEnd,
}

enum BackEnd {
    Ugg,
    Mobalytics,
}
