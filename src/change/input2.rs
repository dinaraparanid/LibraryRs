extern crate fltk;

use crate::{change::Inputable, Lang};
use fltk::{button::Button, dialog::alert, frame::Frame, prelude::*, window::SingleWindow};
use std::{cell::RefCell, rc::Rc};

/// Changes two values

pub struct Input2<I, J>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
{
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    #[allow(dead_code)]
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<I>>,
    #[allow(dead_code)]
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<J>>,
}

impl<I, J> Inputable for Input2<I, J>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
{
    #[inline]
    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>> {
        &self.wind
    }

    #[inline]
    fn set_input(&mut self, lang: Lang) -> Result<Vec<String>, ()> {
        if !InputExt::value(&*((*self.input1).borrow())).is_empty()
            && !InputExt::value(&*((*self.input2).borrow())).is_empty()
        {
            return Ok(vec![
                InputExt::value(&*(self.input1).borrow()),
                InputExt::value(&*(self.input2).borrow()),
            ]);
        } else {
            self.hide();
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Nothing inputted",
                    Lang::Russian => "Ничего не введено",
                },
            );
        }

        InputExt::set_value(&mut *(*self.input1).borrow_mut(), "");
        InputExt::set_value(&mut *(*self.input2).borrow_mut(), "");
        Err(())
    }
}

impl<I, J> Input2<I, J>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
{
    /// Creates window with asking message and 2 input labels

    #[inline]
    pub fn new(title: &str, what_mes1: &str, what_mes2: &str) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, None)));
        win.borrow_mut().set_label(title);

        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, Some("OK"))));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 20, 200, 30, None)));
        wat1.borrow_mut().set_label(what_mes1);

        let inp1: Rc<RefCell<I>> = Rc::new(RefCell::new(WidgetBase::new(150, 20, 300, 30, None)));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 100, 200, 30, None)));
        wat2.borrow_mut().set_label(what_mes2);

        let inp2: Rc<RefCell<J>> = Rc::new(RefCell::new(WidgetBase::new(150, 100, 300, 30, None)));

        WidgetExt::set_label_size(&mut *(*wat1).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat2).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp1).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp2).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Self {
            wind: win,
            ok: but,
            what1: wat1,
            input1: inp1,
            what2: wat2,
            input2: inp2,
        }
    }
}
