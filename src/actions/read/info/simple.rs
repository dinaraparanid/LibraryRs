extern crate fltk;

use crate::{
    actions::{
        book::info::simple::book_info_simple,
        giveaway::simple::{get_book_known_reader, give_book_known_reader},
        read::{add_rem::simple::*, change::simple::*},
        tables::{cell_book2, draw_data, draw_header},
    },
    books::{book_sys::BookSystem, genres::Genres},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    app::App,
    draw,
    enums::{Font, Shortcut},
    frame::Frame,
    group::VGrid,
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table::Table,
    window::SingleWindow,
};

use std::{cell::RefCell, cmp::max, rc::Rc};

/// Messages for info menu
/// for reader_info

#[derive(Clone, Copy)]
enum MessageReader {
    ChangeName,
    ChangeFamily,
    ChangeFather,
    ChangeAge,
    ChangeInfo,
    GiveBook,
    GetBook,
    RemoveThis,
}

/// Show books that reader is reading now.
/// Function that gives information
/// about already known reader

pub fn reader_info_simple_reading(
    ind: usize,
    reader_base: Rc<RefCell<ReaderBase>>,
    book_system: Rc<RefCell<BookSystem>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
    main_table: &mut Table,
) {
    let label = format!(
        "{} {} {}",
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .name
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .family
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .father
        },
    );

    let mut wind = SingleWindow::new(800, 100, 670, 600, None)
        .with_label(label.as_str())
        .center_screen();

    let mut table1 = VGrid::new(0, 0, 670, 200, "");
    table1.set_params(6, 1, 1);

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "First Name",
            Lang::Russian => "\t\tИмя",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .name
        }
    );

    let mut name_frame = Frame::new(10, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Second Name",
            Lang::Russian => "\t\tФамилия",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .family
        }
    );

    let mut family_frame = Frame::new(30, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "\t\tОтчество",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .father
        }
    );

    let mut father_frame = Frame::new(50, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Age",
            Lang::Russian => "\t\tВозраст",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .age()
        }
    );

    let mut age_frame = Frame::new(70, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Additional Info",
            Lang::Russian => "Дополнительная Информация",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .info
        }
    );

    let mut info_frame = Frame::new(200, 50, 100, 30, None).with_label(label.as_str());

    table1.add(&name_frame);
    table1.add(&family_frame);
    table1.add(&father_frame);
    table1.add(&age_frame);
    table1.add(&info_frame);

    let label = format!(
        "{}:",
        match lang {
            Lang::English => "Books read by reader now",
            Lang::Russian => "Книги, читаемые сейчас",
        }
    );

    table1.add(&Frame::new(200, 50, 100, 30, None).with_label(label.as_str()));
    table1.auto_layout();

    let mut table2 = Table::new(0, 230, 670, 600, None);
    table2.set_rows(max(30, unsafe {
        (**(*reader_base).borrow().readers.get_unchecked(ind))
            .borrow()
            .books
            .len() as i32
    }));
    table2.set_row_header(true);
    table2.set_cols(4);
    table2.set_col_header(true);
    table2.set_col_width_all(152);
    table2.end();

    wind.end();

    let mut menu = MenuBar::new(
        0,
        0,
        255 + match lang {
            Lang::English => 0,
            Lang::Russian => 40,
        },
        30,
        "",
    );
    wind.add(&menu);

    let (s, r) = app::channel();

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change name\t",
            Lang::Russian => "&Изменить/Изменить имя\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeName,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change 2-nd name\t",
            Lang::Russian => "&Изменить/Изменить фамилию\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFamily,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change mid name\t",
            Lang::Russian => "&Изменить/Изменить отчество\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFather,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change age\t",
            Lang::Russian => "&Изменить/Изменить возраст\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeAge,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change additional information",
            Lang::Russian => "&Изменить/Изменить дополнительную информацию",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeInfo,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Give book\t",
            Lang::Russian => "&Выдача/Выдать книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::GiveBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Get book\t",
            Lang::Russian => "&Выдача/Вернуть книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::GetBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Remove reader",
            Lang::Russian => "&Удалить читателя",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::RemoveThis,
    );

    wind.show();

    let rb = reader_base.clone();
    let bs = book_system.clone();

    table2.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
        fltk::table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

        fltk::table::TableContext::ColHeader => draw_header(
            &format!(
                "{}",
                match col {
                    0 => match lang {
                        Lang::English => "Title",
                        Lang::Russian => "Название",
                    },

                    1 => match lang {
                        Lang::English => "Author",
                        Lang::Russian => "Автор",
                    },

                    2 => match lang {
                        Lang::English => "Amount of Pages",
                        Lang::Russian => "Кол-во страниц",
                    },

                    _ => match lang {
                        Lang::English => "Number of book",
                        Lang::Russian => "Номер книги",
                    },
                }
            ),
            x,
            y,
            w,
            h,
        ),

        fltk::table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),

        fltk::table::TableContext::Cell => draw_data(
            &format!(
                "{}",
                cell_book2(col, row, ind, &*(*rb).borrow(), &*(*bs).borrow())
            ),
            x,
            y,
            w,
            h,
            t.is_selected(row, col),
            None,
        ),

        _ => (),
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageReader::ChangeName => {
                    if let Some(new_name) = change_name_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        name_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "First Name",
                                    Lang::Russian => "\t\tИмя",
                                },
                                new_name
                            )
                            .as_str(),
                        );
                        name_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeFamily => {
                    if let Some(new_family) = change_family_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        family_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Second Name",
                                    Lang::Russian => "\t\tФамилия",
                                },
                                new_family
                            )
                            .as_str(),
                        );
                        family_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeFather => {
                    if let Some(new_father) = change_father_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        father_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Middle Name",
                                    Lang::Russian => "\t\tОтчество",
                                },
                                new_father
                            )
                            .as_str(),
                        );
                        father_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeAge => {
                    if let Some(new_age) = change_age_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        age_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Age",
                                    Lang::Russian => "\t\tВозраст",
                                },
                                new_age,
                            )
                            .as_str(),
                        );
                        age_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeInfo => {
                    if let Some(new_info) = change_info_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        info_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Additional Info",
                                    Lang::Russian => "Дополнительная Информация",
                                },
                                new_info
                            )
                            .as_str(),
                        );
                        info_frame.redraw();
                    }
                }

                MessageReader::GiveBook => {
                    give_book_known_reader(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    );
                    table2.redraw();
                    main_table.redraw();
                }

                MessageReader::GetBook => {
                    if get_book_known_reader(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        table2.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::RemoveThis => {
                    remove_reader_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        lang,
                    );
                    main_table.redraw();
                    return;
                }
            }
        } else if !wind.shown() {
            return;
        }

        let len = unsafe {
            (**(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .books
                .len()
        };

        (0..len).for_each(|i| {
            if table2.is_selected(i as i32, 0)
                || table2.is_selected(i as i32, 1)
                || table2.is_selected(i as i32, 2)
                || table2.is_selected(i as i32, 3)
            {
                unsafe {
                    book_info_simple(
                        &(**(*reader_base).borrow().readers.get_unchecked(ind))
                            .borrow()
                            .books
                            .get_unchecked(i)
                            .clone(),
                        book_system.clone(),
                        &(*reader_base).borrow(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    );
                }

                table2.unset_selection();
                return;
            }
        });
    }
}

/// Shows all books read by reader.
/// Function that gives information
/// about already known reader

pub(crate) fn reader_info_simple_all_books(
    ind: usize,
    reader_base: Rc<RefCell<ReaderBase>>,
    book_system: Rc<RefCell<BookSystem>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
    main_table: &mut Table,
) {
    let label = format!(
        "{} {} {}",
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .name
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .family
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .father
        },
    );

    let mut wind = SingleWindow::new(800, 100, 670, 600, None)
        .with_label(label.as_str())
        .center_screen();

    let mut table1 = VGrid::new(0, 0, 670, 200, None);
    table1.set_params(6, 1, 1);

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "First Name",
            Lang::Russian => "\t\tИмя",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .name
        }
    );

    let mut name_frame = Frame::new(10, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Second Name",
            Lang::Russian => "\t\tФамилия",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .family
        }
    );

    let mut family_frame = Frame::new(30, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "\t\tОтчество",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .father
        }
    );

    let mut father_frame = Frame::new(50, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Age",
            Lang::Russian => "\t\tВозраст",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .age()
        }
    );

    let mut age_frame = Frame::new(70, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Nearest to return deadline",
            Lang::Russian => "Ближайщее к сдаче",
        },
        if unsafe {
            (**(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .reading
                .is_some()
        } {
            unsafe {
                (*(**(*reader_base).borrow().readers.get_unchecked(ind))
                    .borrow()
                    .reading
                    .as_ref()
                    .unwrap()
                    .first()
                    .unwrap()
                    .upgrade()
                    .unwrap())
                .borrow()
                .to_string(&*(*book_system).borrow())
            }
        } else {
            match lang {
                Lang::English => "None",
                Lang::Russian => "Ничего",
            }
            .to_string()
        }
    );

    let mut reading_frame = Frame::new(90, 50, 100, 30, None).with_label(label.as_str());

    let label = format!(
        "{}: {}",
        match lang {
            Lang::English => "Additional information",
            Lang::Russian => "Дополнительная информация",
        },
        *unsafe {
            &(*(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .info
        }
    );

    let mut info_frame = Frame::new(100, 50, 100, 50, None).with_label(label.as_str());

    table1.add(&name_frame);
    table1.add(&family_frame);
    table1.add(&father_frame);
    table1.add(&age_frame);
    table1.add(&reading_frame);
    table1.add(&info_frame);

    let label = format!(
        "{}:",
        match lang {
            Lang::English => "Books read by reader",
            Lang::Russian => "Прочитанные книги",
        }
    );

    table1.add(&Frame::new(160, 50, 100, 30, None).with_label(label.as_str()));
    table1.auto_layout();

    let mut table2 = Table::new(0, 230, 570, 600, "");
    table2.set_rows(max(30, unsafe {
        (**(*reader_base).borrow().readers.get_unchecked(ind))
            .borrow()
            .books
            .len() as i32
    }));
    table2.set_row_header(true);
    table2.set_cols(4);
    table2.set_col_header(true);
    table2.set_col_width_all(152);
    table2.end();

    wind.end();

    let mut menu = MenuBar::new(
        0,
        0,
        255 + match lang {
            Lang::English => 0,
            Lang::Russian => 40,
        },
        30,
        "",
    );
    wind.add(&menu);

    let (s, r) = app::channel();

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change name\t",
            Lang::Russian => "&Изменить/Изменить имя\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeName,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change 2-nd name\t",
            Lang::Russian => "&Изменить/Изменить фамилию\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFamily,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change mid name\t",
            Lang::Russian => "&Изменить/Изменить отчество\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFather,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change age\t",
            Lang::Russian => "&Изменить/Изменить возраст\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeAge,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change additional information",
            Lang::Russian => "&Изменить/Изменить дополнительную информацию",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeInfo,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Give book\t",
            Lang::Russian => "&Выдача/Выдать книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::GiveBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Get book\t",
            Lang::Russian => "&Выдача/Вернуть книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::GetBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Remove reader",
            Lang::Russian => "&Удалить читателя",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::RemoveThis,
    );

    wind.show();

    let rb = reader_base.clone();
    let bs = book_system.clone();

    table2.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
        fltk::table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

        fltk::table::TableContext::ColHeader => draw_header(
            &format!(
                "{}",
                match col {
                    0 => match lang {
                        Lang::English => "Title",
                        Lang::Russian => "Название",
                    },

                    1 => match lang {
                        Lang::English => "Author",
                        Lang::Russian => "Автор",
                    },

                    2 => match lang {
                        Lang::English => "Amount of Pages",
                        Lang::Russian => "Кол-во страниц",
                    },

                    _ => match lang {
                        Lang::English => "Number of book",
                        Lang::Russian => "Номер книги",
                    },
                }
            ),
            x,
            y,
            w,
            h,
        ),

        fltk::table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),

        fltk::table::TableContext::Cell => draw_data(
            &format!(
                "{}",
                cell_book2(col, row, ind, &*(*rb).borrow(), &*(*bs).borrow())
            ),
            x,
            y,
            w,
            h,
            t.is_selected(row, col),
            None,
        ),

        _ => (),
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageReader::ChangeName => {
                    if let Some(new_name) = change_name_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        name_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "First Name",
                                    Lang::Russian => "\t\tИмя",
                                },
                                new_name
                            )
                            .as_str(),
                        );
                        name_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeFamily => {
                    if let Some(new_family) = change_family_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        family_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Second Name",
                                    Lang::Russian => "\t\tФамилия",
                                },
                                new_family
                            )
                            .as_str(),
                        );
                        family_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeFather => {
                    if let Some(new_father) = change_father_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        father_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Middle Name",
                                    Lang::Russian => "\t\tОтчество",
                                },
                                new_father
                            )
                            .as_str(),
                        );
                        father_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeAge => {
                    if let Some(new_age) = change_age_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        age_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Age",
                                    Lang::Russian => "\t\tВозраст",
                                },
                                new_age,
                            )
                            .as_str(),
                        );
                        age_frame.redraw();
                        main_table.redraw();
                    }
                }

                MessageReader::ChangeInfo => {
                    if let Some(new_info) = change_info_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        info_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Additional Info",
                                    Lang::Russian => "Дополнительная Информация",
                                },
                                new_info
                            )
                            .as_str(),
                        );
                        info_frame.redraw();
                    }
                }

                MessageReader::GiveBook => {
                    if let Some(book) = give_book_known_reader(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        reading_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Reading now",
                                    Lang::Russian => "Читается сейчас",
                                },
                                book
                            )
                            .as_str(),
                        );
                        reading_frame.redraw();
                    }
                    table2.redraw();
                    main_table.redraw();
                }

                MessageReader::GetBook => {
                    if get_book_known_reader(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        reading_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Reading now",
                                    Lang::Russian => "Читается сейчас",
                                },
                                match lang {
                                    Lang::English => "None",
                                    Lang::Russian => "Ничего",
                                },
                            )
                            .as_str(),
                        );
                        reading_frame.redraw();
                    }
                    table2.redraw();
                    main_table.redraw();
                }

                MessageReader::RemoveThis => {
                    remove_reader_simple(
                        ind,
                        &mut *(*reader_base).borrow_mut(),
                        &mut *(*book_system).borrow_mut(),
                        genres,
                        caretaker,
                        lang,
                    );
                    return;
                }
            }
        } else if !wind.shown() {
            return;
        }

        let len = unsafe {
            (**(*reader_base).borrow().readers.get_unchecked(ind))
                .borrow()
                .books
                .len()
        };

        (0..len).for_each(|i| {
            if table2.is_selected(i as i32, 0)
                || table2.is_selected(i as i32, 1)
                || table2.is_selected(i as i32, 2)
                || table2.is_selected(i as i32, 3)
            {
                unsafe {
                    book_info_simple(
                        &(**(*reader_base).borrow().readers.get_unchecked(ind))
                            .borrow()
                            .books
                            .get_unchecked(i)
                            .clone(),
                        book_system.clone(),
                        &(*reader_base).borrow(),
                        genres,
                        caretaker,
                        app,
                        lang,
                    );
                }

                table2.unset_selection();
                return;
            }
        });
    }
}
