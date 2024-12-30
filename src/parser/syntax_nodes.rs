
use super::token_nodes::Token;

#[derive(Clone)]
pub enum SyntaxNode {

    /* Expression nodes */
    NamedExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    TestExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    LambdaExprNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Box<Token>, Box<SyntaxNode>, bool),
    OrTestExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    AndTestExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    NotTestExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    CompareLessExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareLessEqualExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareGreaterExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareGreaterEqualExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareEqualExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareNotEqualExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareInEqualExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareNotInExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<Token>, Box<SyntaxNode>),
    CompareIsExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    CompareIsNotExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<Token>, Box<SyntaxNode>),
    StarExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    OrExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    XorExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    AndExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ShiftLeftExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ShiftRightExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    PlusExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    MinusExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    MulExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    DivExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ModuloExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    MatricesExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    FloorDivExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    UnaryPlusExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    UnaryMinusExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    UnaryBitInvertExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    PowerExprNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    AtomExprNode(u32, u32, Option<Box<Token>>, Box<SyntaxNode>, Vec<Box<SyntaxNode>>),
    NameExprNode(u32, u32, Box<Token>),
    NumberExprNode(u32, u32, Box<Token>),
    StringExprNode(u32, u32, Vec<Box<Token>>),
    EllipsisExprNode(u32, u32, Box<Token>),
    NoneExprNode(u32, u32, Box<Token>),
    FalseExprNode(u32, u32, Box<Token>),
    TrueExprNode(u32, u32, Box<Token>),
    TupleExprNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Box<Token>),
    ListExprNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Box<Token>),
    DictionaryExprNode(u32, u32, Box<Token>, Vec<Box<SyntaxNode>>, Vec<Box<Token>>, Box<Token>),
    SetExprNode(u32, u32, Box<Token>, Vec<Box<SyntaxNode>>, Vec<Box<Token>>, Box<Token>),
    DictionaryEntryNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    DictionaryReferenceNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    SetReferenceNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    TestListComprehensionExprNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    TrailerDotNameExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    TrailerCallExprNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Box<Token>),
    TrailerIndexExprNode(u32, u32, Box<Token>, Box<SyntaxNode>, Box<Token>),
    SubscriptListExprNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    SubscriptExprNode(u32, u32, Option<Box<SyntaxNode>>, Option<Box<Token>>, Option<Box<SyntaxNode>>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    ExprListExprNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    TestListExprNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    ArgListExprNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    ArgumentExprNode(u32, u32, Option<Box<SyntaxNode>>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    MulArgumentExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    PowerArgumentExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    SyncCompForExprNode(u32, u32, Box<Token>, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>, Option<Box<SyntaxNode>>),
    CompForExprNode(u32, u32, Box<Token>, Box<SyntaxNode>, Option<Box<SyntaxNode>>),
    CompIfExprNode(u32, u32, Box<Token>, Box<SyntaxNode>, Option<Box<SyntaxNode>>),
    YieldExprNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    YieldFromExprNode(u32, u32, Box<Token>, Box<Token>, Box<SyntaxNode>),

    /* Statement nodes */
    SimpleStmtNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>, Box<Token>),
    DelStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    PassStmtNode(u32, u32, Box<Token>),
    BreakStmtNode(u32, u32, Box<Token>),
    ContinueStmtNode(u32, u32, Box<Token>),
    ReturnStmtNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>),
    RaiseStmtNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    GlobalStmtNode(u32, u32, Box<Token>, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    NonlocalStmtNode(u32, u32, Box<Token>, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    AssertStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    DottedNameStmtNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    DottedAsNamesStmtNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    ImportAsNamesStmtNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    DottedAsNameStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ImportAsNameStmtNode(u32, u32, Box<Token>, Option<Box<Token>>, Option<Box<Token>>),
    ImportNameStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    ImportFromStmtNode(u32, u32, Box<Token>, Vec<Box<Token>>, Option<Box<SyntaxNode>>, Box<Token>, Option<Box<Token>>, Option<Box<SyntaxNode>>, Option<Box<Token>>),
    TestListStarExprStmtNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>),
    PlusAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    MinusAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    MulAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    DivAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ModuloAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    MatricesAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    FloorDivAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    BitAndAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    BitOrAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    BitXorAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ShiftLeftAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ShiftRightAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    PowerAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    AnnAssignStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    AssignmentStmtNode(u32, u32, Box<SyntaxNode>, Vec<Box<SyntaxNode>>, Option<Box<Token>>),
    AssignmentElementStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    AsyncStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>),
    IfStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>, Vec<Box<SyntaxNode>>, Option<Box<SyntaxNode>>),
    ElifStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    ElseStmtNode(u32, u32, Box<Token>, Box<Token>, Box<SyntaxNode>),
    WhileStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>, Option<Box<SyntaxNode>>),
    ForStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>, Box<Token>, Option<Box<Token>>, Box<SyntaxNode>, Option<Box<SyntaxNode>>),
    WithStmtNode(u32, u32, Box<Token>, Vec<Box<SyntaxNode>>, Vec<Box<Token>>, Box<Token>, Option<Box<Token>>, Box<SyntaxNode>),
    WithItemStmtNode(u32, u32, Box<SyntaxNode>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    SuiteStmtNode(u32, u32, Box<Token>, Box<Token>, Vec<Box<SyntaxNode>>, Box<Token>),
    ExceptClauseStmtNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    TryStmtNode(u32, u32, Box<Token>, Box<Token>, Box<SyntaxNode>, Vec<Box<SyntaxNode>>, Option<Box<SyntaxNode>>, Option<Box<Token>>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    ExceptStmtNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),

    /* Block nodes */
    EvalInputStmtNode(u32, u32, Box<SyntaxNode>, Vec<Box<Token>>, Box<Token>),
    FuncTypeInputStmtNode(u32, u32, Box<SyntaxNode>, Vec<Box<Token>>, Box<Token>),
    FileInputStmtNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>, Box<Token>),
    SingleInputStmtNode(u32, u32, Option<Box<SyntaxNode>>, Option<Box<Token>>),

    DecoratedStmtNode(u32, u32, Box<SyntaxNode>, Box<SyntaxNode>),
    DecoratorsStmtNode(u32, u32, Vec<Box<SyntaxNode>>),
    DecoratorStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Option<Box<Token>>, Option<Box<SyntaxNode>>, Option<Box<Token>>, Box<Token>),
    FuncDefinitionNode(u32, u32, Box<Token>, Box<Token>, Box<SyntaxNode>, Option<Box<Token>>, Option<Box<SyntaxNode>>, Box<Token>, Option<Box<Token>>, Box<SyntaxNode>),


    FuncTypeStmtNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Box<Token>, Box<Token>, Box<SyntaxNode>),
    TypeListStmtNode(u32, u32, Vec<Box<SyntaxNode>>, Vec<Box<Token>>, Option<Box<Token>>, Option<Box<SyntaxNode>>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    FuncBodyStmtNode(u32, u32, Box<Token>, Option<Box<Token>>, Option<Box<Token>>, Box<Token>, Vec<Box<SyntaxNode>>, Box<Token>),
    ClassDefStmtNode(u32, u32, Box<Token>, Box<Token>, Option<Box<Token>>, Option<Box<SyntaxNode>>, Option<Box<Token>>, Box<Token>, Box<SyntaxNode>),
    ParametersNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>, Box<Token>),
    TypedFormalParameterNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),

    TypedListNode(u32, u32, Vec::<Box<SyntaxNode>>, Vec::<Box<Token>>, Option<Box<Token>>),


    VarListNode(u32, u32, Vec::<Box<SyntaxNode>>, Vec::<Box<Token>>, Option<Box<Token>>),
    VarElementNode(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    VarStarElementNode(u32, u32, Box<Token>, Option<Box<SyntaxNode>>),
    VarPowerElementNode(u32, u32, Box<Token>, Box<SyntaxNode>),

    /* Matches nodes */
    MatchStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Box<Token>, Box<Token>, Box<Token>, Vec<Box<SyntaxNode>>, Box<Token>),
    SubjectExprNode(u32, u32, Box<SyntaxNode>, Option<Box<Token>>, Option<Box<SyntaxNode>>),
    CaseElementStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>, Option<Box<SyntaxNode>>, Box<Token>, Box<SyntaxNode>),
    GuardElementStmtNode(u32, u32, Box<Token>, Box<SyntaxNode>),


    MatchAsPattern(u32, u32, Box<SyntaxNode>, Box<Token>, Box<SyntaxNode>),
    MatchOrPatterns(u32, u32, Box<SyntaxNode>, Vec<Box<Token>>, Vec<Box<SyntaxNode>>),

    DefaultPatterNode(u32, u32, Box<Token>),

    MappingPatternNode(u32, u32, Box<Token>, Vec<Box<SyntaxNode>>, Vec<Box<Token>>,Box<Token>),
    KeywordPatternNode(u32, u32, Box<Token>, Box<Token>, Box<SyntaxNode>),
}
