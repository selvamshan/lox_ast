

fn test_print() {
    let expression = Expr::Binary( BinaryExpr {
        left: Box::new( Expr::Unary( UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new( Expr::Literal( LiteralExpr {
                value: Some( Object::Num(123.0) )
            }) )
        }) ),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new( Expr::Grouping( GroupingExpr {
            expression: Box::new( Expr::Literal( LiteralExpr {
                value: Some( Object::Num(45.67) )
            }) )
        }) )
    });

    let mut printer = AstPrinter;
    match printer.print(&expression) {
        Ok(result) => println!("{}", result),
        Err(e) => e.report("".to_string()),
    }
}