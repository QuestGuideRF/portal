pub fn shader_error_parser(error: &str) -> Vec<Result<(usize, &str), &str>> {
    fn expect_str(input: &mut &str, to_expect: &str) -> Option<()> {
        if to_expect.chars().count() > input.chars().count() {
            return None;
        }
        if input.chars().zip(to_expect.chars()).any(|(a, b)| a != b) {
            return None;
        }
        *input = &input[to_expect.len()..];
        Some(())
    }
    fn expect_int(input: &mut &str) -> Option<usize> {
        let pos = input
            .char_indices()
            .take_while(|(_, c)| c.is_ascii_digit())
            .last()
            .map(|(i, c)| i + c.len_utf8())?;
        let lineno: usize = input[..pos].parse().ok()?;
        *input = &input[pos..];
        Some(lineno)
    }
    fn try_parse_1(mut line: &str) -> Option<(usize, &str)> {
        expect_str(&mut line, "0(")?;
        let lineno = expect_int(&mut line)?;
        expect_str(&mut line, ") : ошибка ")?;
        Some((lineno, line))
    }
    fn try_parse_2(mut line: &str) -> Option<(usize, &str)> {
        expect_str(&mut line, "0(")?;
        let lineno = expect_int(&mut line)?;
        expect_str(&mut line, ") : предупреждение ")?;
        Some((lineno, line))
    }
    fn try_parse_3(mut line: &str) -> Option<(usize, &str)> {
        expect_str(&mut line, "ОШИБКА: 0:")?;
        let lineno = expect_int(&mut line)?;
        expect_str(&mut line, ": ")?;
        Some((lineno, line))
    }
    error
        .split('\n')
        .map(|line| {
            if let Some(r) = try_parse_1(line) {
                Ok(r)
            } else if let Some(r) = try_parse_2(line) {
                Ok(r)
            } else if let Some(r) = try_parse_3(line) {
                Ok(r)
            } else {
                macroquad::prelude::miniquad::error!("не могу разобрать строку: `{}`", line);
                Err(line)
            }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shader_error_parser_test() {
        let linux1 = r#"0(270): ошибка C0000: синтаксическая ошибка, неожиданный '}' в токене "}"
0(286): ошибка C1503: неопределенная переменная "a"
0(286): ошибка C1503: неопределенная переменная "n"
0(287): ошибка C1503: неопределенная переменная "b"
0(287): ошибка C1503: неопределенная переменная "n"
0(288): ошибка C0000: синтаксическая ошибка, неожиданное зарезервированное слово. "return" по жетону "return"
0(327): ошибка C1503: неопределенная переменная "two_lines_nearest_points"
0(327): ошибка C1503: неопределенная переменная "l"
0(327): ошибка C1503: неопределенная переменная "r"
0(329): ошибка C1503: неопределенная переменная "l"
0(329): ошибка C1503: неопределенная переменная "l"
0(330): ошибка C1503: неопределенная переменная "r"
0(330): ошибка C1503: неопределенная переменная "r"
0(332): ошибка C1059: неконстантное выражение при инициализации
0(334): ошибка C0000: синтаксическая ошибка, неожиданное зарезервированное слово. "if" по жетону "if"
0(347): ошибка C1503: неопределенная переменная "u"
0(348): ошибка C1503: неопределенная переменная "u"
0(349): ошибка C1503: неопределенная переменная "u"
0(350): ошибка C0000: синтаксическая ошибка, неожиданное зарезервированное слово "return" по жетону "return"
0(359): ошибка C1503: неопределенная переменная "u"
0(359): ошибка C1038: объявление "b" конфликтует с предыдущим объявлением в 0(347)
0(360): ошибка C1503: неопределенная переменная "u"
0(360): ошибка C1038: объявление "c" конфликтует с предыдущим объявлением в 0(348)
0(361): ошибка C1503: неопределенная переменная "u"
0(361): ошибка C1038: объявление "d" конфликтует с предыдущим объявлением в 0(349)
0(362): ошибка C0000: синтаксическая ошибка, неожиданное зарезервированное слово "return" по жетону "return"
0(373): ошибка C0000: синтаксическая ошибка, неожиданный '}' в токене "}"
0(375): ошибка C0000: синтаксическая ошибка, неожиданный '(', ожидание "::" по жетону "("
0(378): ошибка C1503: неопределенная переменная "mobius_step"
0(378): ошибка C1503: неопределенная переменная "r"
0(379): ошибка C0000: синтаксическая ошибка, неожиданное зарезервированное слово. "for" по жетону "for"
0(433): ошибка C1503: неопределенная переменная "op"
0(433): ошибка C1503: неопределенная переменная "r"
0(433): ошибка C1038: объявление "b" конфликтует с предыдущим объявлением в 0(347)
0(434): ошибка C1503: неопределенная переменная "op"
0(434): ошибка C1503: неопределенная переменная "op"
0(435): ошибка C0000: синтаксическая ошибка, неожиданное зарезервированное слово. "return" по жетону "return"
0(550): ошибка C1503: неопределенная переменная "is_inside_triangle"
0(555): ошибка C1503: неопределенная переменная "is_inside_triangle"
0(631): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(635): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(639): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(643): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(647): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(651): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(655): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(659): ошибка C1503: неопределенная переменная "process_plane_intersection"
0(664): ошибка C1503: неопределенная переменная "process_portal_intersection"
0(668): ошибка C1503: неопределенная переменная "process_portal_intersection"
0(673): ошибка C1503: неопределенная переменная "process_portal_intersection"
0(677): ошибка C1503: неопределенная переменная "process_portal_intersection"
0(680): ошибка C1503: неопределенная переменная "a2_mat"
0(682): ошибка C1503: неопределенная переменная "process_portal_intersection"
0(686): ошибка C1503: неопределенная переменная "process_portal_intersection""#;
        assert!(shader_error_parser(linux1).iter().all(|x| x.is_ok()));
        let linux2 = r#"0(277): ошибка C1503: неопределенная переменная "borer_m"
0(292): ошибка C0000: синтаксическая ошибка, неожиданное '}', ожидание ',' или ';' по жетону "}"
0(284): ошибка C1110: функция "two_lines_nearest_points" не имеет оператора возврата
0(295): ошибка C1115: невозможно найти совместимую перегруженную функцию. "dot(mat3, vec3)"
0(299): ошибка C1102: несовместимый тип параметра №1 ("a.84")"#;
        assert!(shader_error_parser(linux2).iter().all(|x| x.is_ok()));
        let linux3 = r#"0(365): предупреждение C7022: нераспознанный спецификатор профиля "a""#;
        assert!(shader_error_parser(linux3).iter().all(|x| x.is_ok()));
        let web_linux = r#"ERROR: 0:565: 'pos' : redefinition
ERROR: 0:586: 'pos' : redefinition
ERROR: 0:606: 'pos' : redefinition
ERROR: 0:607: '<' : comparison operator only defined for scalars
ERROR: 0:607: '<' : wrong operand types - no operation '<' exists that takes a left-hand operand of type 'in highp 4-component vector of float' and a right operand of type 'const float' (or there is no acceptable conversion)
ERROR: 0:613: '<' : comparison operator only defined for scalars
ERROR: 0:613: '<' : wrong operand types - no operation '<' exists that takes a left-hand operand of type 'in highp 4-component vector of float' and a right operand of type 'const float' (or there is no acceptable conversion)"#;
        assert!(shader_error_parser(web_linux).iter().all(|x| x.is_ok()));
    }
}