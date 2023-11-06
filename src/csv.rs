#[macro_export]
macro_rules! csv_parser {
    ($vis:vis $name:ident, $entryname:ident, $ename:ident
        $(
            $cident:ident, $cename:ident, $cname:literal : $cty:ty
        ),+
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis struct $name(pub Vec<$entryname>);

        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis struct $entryname {
            $(
                pub $cident: $cty
            ),+
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis enum $ename {
            InvalidHeader,
            UnexpectedEOF,
            UnexpectedEOL,
            $(
                $cename(<$cty as ::std::str::FromStr>::Err)
            ),+
        }

        impl ::std::str::FromStr for $name {
            type Err = $ename;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut lines = s.split('\n');

                let header = lines.next().ok_or($ename::UnexpectedEOF)?;

                if format!("{},", header) != concat!($($cname,","),+) {
                    return Err($ename::InvalidHeader);
                }

                let mut data = vec![];

                for line in lines {
                    let mut entries = line.split(',');
                    $(
                        let $cident = {
                            let entry = entries.next().ok_or($ename::UnexpectedEOL)?;
                            <$cty as ::std::str::FromStr>::from_str(entry).map_err($ename::$cename)?
                        };
                    )+

                    let entry = $entryname {
                        $($cident),+
                    };

                    data.push(entry);
                }

                Ok($name(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::csv_parser;

    csv_parser!(
        pub Csv, CsvRow, CsvParseError
        field_one, FieldOneError, 
            "field one":    String,
        field_two, FieldTwoError, 
            "field two":    isize,
        field_three, FieldThreeError, 
            "field three":    usize
    );

    #[test]
    fn valid_csv() {
        assert_eq!(<Csv as FromStr>::from_str(r#"field one,field two,field three
test1,-2512,123
test2,123232,99
,0,0"#), Ok(Csv(vec![
            CsvRow {
                field_one: "test1".to_string(),
                field_two: -2512,
                field_three: 123,
            },
            CsvRow {
                field_one: "test2".to_string(),
                field_two: 123232,
                field_three: 99
            },
            CsvRow {
                field_one: "".to_string(),
                field_two: 0,
                field_three: 0
            }
        ])));
    }
}