use std::fmt;

#[derive(Debug)]
enum BodyPartType {
    Head,
    Body,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Debug)]
pub struct BodyPart {
    visible: bool,
    visible_output: String,
    invisible_output: String, 
}

impl BodyPart {
    fn new(body_part_type: BodyPartType) -> Self {
        match body_part_type {
            BodyPartType::Head => BodyPart { 
                visible: false,
                visible_output: "(_)".to_string(),
                invisible_output: "   ".to_string(),
            },
            BodyPartType::Body => BodyPart { 
                visible: false,
                visible_output: "|".to_string(),
                invisible_output: " ".to_string(),
            },
            BodyPartType::LeftArm => BodyPart { 
                visible: false,
                visible_output: "\\".to_string(),
                invisible_output: " ".to_string(),
            },
            BodyPartType::RightArm => BodyPart { 
                visible: false,
                visible_output: "/".to_string(),
                invisible_output: " ".to_string(),
            },
            BodyPartType::LeftLeg => BodyPart { 
                visible: false,
                visible_output: "/".to_string(),
                invisible_output: " ".to_string(),
            },
            BodyPartType::RightLeg => BodyPart { 
                visible: false,
                visible_output: "\\".to_string(),
                invisible_output: " ".to_string(),
            },
        }
            
    }
}

impl fmt::Display for BodyPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.visible {
            write!(f, "{}", self.visible_output)
        } else {
            write!(f, "{}", self.invisible_output)
        }
    }
}


#[derive(Debug)]
pub struct Body {
    pub head: BodyPart,
    pub body: BodyPart,
    pub left_arm: BodyPart,
    pub right_arm: BodyPart,
    pub left_leg: BodyPart,
    pub right_leg: BodyPart,
}

impl Body {
    pub fn new() -> Self {
        Body {
            head: BodyPart::new(BodyPartType::Head),
            body: BodyPart::new(BodyPartType::Body),
            left_arm: BodyPart::new(BodyPartType::LeftArm),
            right_arm: BodyPart::new(BodyPartType::RightArm),
            left_leg: BodyPart::new(BodyPartType::LeftLeg),
            right_leg: BodyPart::new(BodyPartType::RightLeg),
        }
    }

    pub fn reveal(&mut self) {
        if !self.head.visible {
            self.head.visible = true;
        } else if !self.body.visible {
            self.body.visible = true;
        } else if !self.left_arm.visible {
            self.left_arm.visible = true;
        } else if !self.right_arm.visible {
            self.right_arm.visible = true;
        } else if !self.left_leg.visible {
            self.left_leg.visible = true;
        } else if !self.right_leg.visible {
            self.right_leg.visible = true;
        }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "    _______     ")?;
        writeln!(f, "    |/      |   ")?;
        writeln!(f, "    |      {}  ", self.head)?;
        writeln!(f, "    |      {}{}{}  ", self.left_arm, self.body, self.right_arm)?;
        writeln!(f, "    |       {}   ", self.body)?;
        writeln!(f, "    |      {} {}  ", self.left_leg, self.right_leg)?;
        writeln!(f, "    |           ")?;
        writeln!(f, "jgs_|___        ")
    }
}

