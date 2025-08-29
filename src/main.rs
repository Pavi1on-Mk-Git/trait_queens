use std::marker::PhantomData;

struct Nil;
struct Cons<X, Xs>(PhantomData<X>, PhantomData<Xs>);

trait First {
    type X;
}

impl First for Nil {
    type X = Nil;
}

impl<X, Xs> First for Cons<X, Xs> {
    type X = X;
}

trait ListConcat<B> {
    type C;
}

impl<X> ListConcat<X> for Nil {
    type C = X;
}

impl<Bs, A, As: ListConcat<Bs>> ListConcat<Bs> for Cons<A, As> {
    type C = Cons<A, <As as ListConcat<Bs>>::C>;
}

trait ListConcatAll {
    type L;
}

impl ListConcatAll for Nil {
    type L = Nil;
}

impl<Chunk: ListConcat<<Rest as ListConcatAll>::L>, Rest: ListConcatAll> ListConcatAll
    for Cons<Chunk, Rest>
{
    type L = <Chunk as ListConcat<<Rest as ListConcatAll>::L>>::C;
}

trait AnyTrue {
    type T;
}

impl AnyTrue for Nil {
    type T = False;
}

impl<More> AnyTrue for Cons<True, More> {
    type T = True;
}

impl<List: AnyTrue> AnyTrue for Cons<False, List> {
    type T = <List as AnyTrue>::T;
}

struct True;
struct False;

trait Not {
    type B;
}

impl Not for False {
    type B = True;
}

impl Not for True {
    type B = False;
}

trait Or<B2> {
    type B;
}

impl Or<True> for True {
    type B = True;
}

impl Or<True> for False {
    type B = True;
}

impl Or<False> for True {
    type B = True;
}

impl Or<False> for False {
    type B = False;
}

struct Z;
struct S<N>(PhantomData<N>);

trait PeanoEqual<B> {
    type T;
}

impl PeanoEqual<Z> for Z {
    type T = True;
}

impl<A> PeanoEqual<Z> for S<A> {
    type T = False;
}

impl<B> PeanoEqual<S<B>> for Z {
    type T = False;
}

impl<A: PeanoEqual<B>, B> PeanoEqual<S<B>> for S<A> {
    type T = <A as PeanoEqual<B>>::T;
}

trait PeanoLT<B> {
    type T;
}

impl PeanoLT<Z> for Z {
    type T = False;
}

impl<X> PeanoLT<Z> for S<X> {
    type T = False;
}

impl<X> PeanoLT<S<X>> for Z {
    type T = True;
}

impl<A: PeanoLT<B>, B> PeanoLT<S<B>> for S<A> {
    type T = <A as PeanoLT<B>>::T;
}

trait PeanoAbsDiff<B> {
    type C;
}

impl PeanoAbsDiff<Z> for Z {
    type C = Z;
}

impl<B> PeanoAbsDiff<S<B>> for Z {
    type C = S<B>;
}

impl<A> PeanoAbsDiff<Z> for S<A> {
    type C = S<A>;
}

impl<A: PeanoAbsDiff<B>, B> PeanoAbsDiff<S<B>> for S<A> {
    type C = <A as PeanoAbsDiff<B>>::C;
}

trait Range {
    type Xs;
}

impl Range for Z {
    type Xs = Nil;
}

impl<N: Range> Range for S<N> {
    type Xs = Cons<N, <N as Range>::Xs>;
}

trait Apply<A> {
    type R;
}

struct Conj1<List>(PhantomData<List>);

impl<X, List> Apply<X> for Conj1<List> {
    type R = Cons<X, List>;
}

trait Map<Xs> {
    type Ys;
}

impl<F> Map<Nil> for F {
    type Ys = Nil;
}

impl<F: Apply<X> + Map<Xs>, X, Xs> Map<Cons<X, Xs>> for F {
    type Ys = Cons<<F as Apply<X>>::R, <F as Map<Xs>>::Ys>;
}

trait MapCat<Xs> {
    type Zs;
}

impl<F> MapCat<Nil> for F {
    type Zs = Nil;
}

impl<F: Map<Cons<X, Xs>>, X, Xs> MapCat<Cons<X, Xs>> for F
where
    <F as Map<Cons<X, Xs>>>::Ys: ListConcatAll,
{
    type Zs = <<F as Map<Cons<X, Xs>>>::Ys as ListConcatAll>::L;
}

trait AppendIf<X, Ys> {
    type Zs;
}

impl<X, Ys> AppendIf<X, Ys> for True {
    type Zs = Cons<X, Ys>;
}

impl<X, Ys> AppendIf<X, Ys> for False {
    type Zs = Ys;
}

trait Filter<F> {
    type Ys;
}

impl<F> Filter<F> for Nil {
    type Ys = Nil;
}

impl<F: Apply<X>, X, Xs: Filter<F>> Filter<F> for Cons<X, Xs>
where
    <F as Apply<X>>::R: AppendIf<X, <Xs as Filter<F>>::Ys>,
{
    type Ys = <<F as Apply<X>>::R as AppendIf<X, <Xs as Filter<F>>::Ys>>::Zs;
}

struct Queen<X, Y>(PhantomData<X>, PhantomData<Y>);

struct Queen1<X>(PhantomData<X>);

impl<X, Y> Apply<Y> for Queen1<X> {
    type R = Queen<X, Y>;
}

trait QueensInRow<X> {
    type Queens;
}

impl<N: Range, X> QueensInRow<X> for N
where
    Queen1<X>: Map<<N as Range>::Xs>,
{
    type Queens = <Queen1<X> as Map<<N as Range>::Xs>>::Ys;
}

trait Threatens<B> {
    type T;
}

impl<Ax: PeanoEqual<Bx> + PeanoAbsDiff<Bx>, Ay: PeanoEqual<By> + PeanoAbsDiff<By>, Bx, By>
    Threatens<Queen<Bx, By>> for Queen<Ax, Ay>
where
    <Ax as PeanoEqual<Bx>>::T: Or<<Ay as PeanoEqual<By>>::T>,
    <Ax as PeanoAbsDiff<Bx>>::C: PeanoEqual<<Ay as PeanoAbsDiff<By>>::C>,
    <<Ax as PeanoEqual<Bx>>::T as Or<<Ay as PeanoEqual<By>>::T>>::B:
        Or<<<Ax as PeanoAbsDiff<Bx>>::C as PeanoEqual<<Ay as PeanoAbsDiff<By>>::C>>::T>,
{
    type T = <<<Ax as PeanoEqual<Bx>>::T as Or<<Ay as PeanoEqual<By>>::T>>::B as Or<
        <<Ax as PeanoAbsDiff<Bx>>::C as PeanoEqual<<Ay as PeanoAbsDiff<By>>::C>>::T,
    >>::B;
}

struct Threatens1<A>(PhantomData<A>);

impl<A: Threatens<B>, B> Apply<B> for Threatens1<A> {
    type R = <A as Threatens<B>>::T;
}

trait Safe<QueenT> {
    type T;
}

impl<Config, QueenT> Safe<QueenT> for Config
where
    Threatens1<QueenT>: Map<Config>,
    <Threatens1<QueenT> as Map<Config>>::Ys: AnyTrue,
    <<Threatens1<QueenT> as Map<Config>>::Ys as AnyTrue>::T: Not,
{
    type T = <<<Threatens1<QueenT> as Map<Config>>::Ys as AnyTrue>::T as Not>::B;
}

struct Safe1<Config>(PhantomData<Config>);

impl<Config: Safe<QueenT>, QueenT> Apply<QueenT> for Safe1<Config> {
    type R = <Config as Safe<QueenT>>::T;
}

trait AddQueen<X, C> {
    type Cs;
}

impl<N: Range, X, C> AddQueen<X, C> for N
where
    Queen1<X>: Map<<N as Range>::Xs>,
    <Queen1<X> as Map<<N as Range>::Xs>>::Ys: Filter<Safe1<C>>,
    Conj1<C>: Map<<<Queen1<X> as Map<<N as Range>::Xs>>::Ys as Filter<Safe1<C>>>::Ys>,
{
    type Cs = <Conj1<C> as Map<<<N as QueensInRow<X>>::Queens as Filter<Safe1<C>>>::Ys>>::Ys;
}

struct AddQueen2<N, X>(PhantomData<N>, PhantomData<X>);

impl<N: AddQueen<X, C>, X, C> Apply<C> for AddQueen2<N, X> {
    type R = <N as AddQueen<X, C>>::Cs;
}

trait AddQueenToAll<X, Cs> {
    type Cs2;
}

impl<N, X, Cs> AddQueenToAll<X, Cs> for N
where
    AddQueen2<N, X>: MapCat<Cs>,
{
    type Cs2 = <AddQueen2<N, X> as MapCat<Cs>>::Zs;
}

trait AddQueensIf<N, X, Cs> {
    type Cs2;
}

impl<N, X, Cs> AddQueensIf<N, X, Cs> for False {
    type Cs2 = Cs;
}

impl<N, X, Cs> AddQueensIf<N, X, Cs> for True
where
    AddQueen2<N, X>: MapCat<Cs>,
    S<X>: PeanoLT<N>,
    <S<X> as PeanoLT<N>>::T: AddQueensIf<N, S<X>, <AddQueen2<N, X> as MapCat<Cs>>::Zs>,
{
    type Cs2 = <N as AddQueens<S<X>, <N as AddQueenToAll<X, Cs>>::Cs2>>::Cs2;
}
trait AddQueens<X, Cs> {
    type Cs2;
}

impl<N, X: PeanoLT<N>, Cs> AddQueens<X, Cs> for N
where
    <X as PeanoLT<N>>::T: AddQueensIf<N, X, Cs>,
{
    type Cs2 = <<X as PeanoLT<N>>::T as AddQueensIf<N, X, Cs>>::Cs2;
}

trait Solution<N> {
    type C;
}

impl<N> Solution<N> for ()
where
    Z: PeanoLT<N>,
    <Z as PeanoLT<N>>::T: AddQueensIf<N, Z, Cons<Nil, Nil>>,
    <<Z as PeanoLT<N>>::T as AddQueensIf<N, Z, Cons<Nil, Nil>>>::Cs2: First,
{
    type C = <<N as AddQueens<Z, Cons<Nil, Nil>>>::Cs2 as First>::X;
}

trait Printable {
    fn print();
}

impl<X: Printable, Xs: Printable> Printable for Cons<X, Xs> {
    fn print() {
        print!("Cons(");
        X::print();
        print!(", ");
        Xs::print();
        print!(")");
    }
}

impl Printable for Nil {
    fn print() {
        print!("Nil");
    }
}

impl Printable for Z {
    fn print() {
        print!("0")
    }
}

impl<N: Printable> Printable for S<N> {
    fn print() {
        N::print();
        print!(" + 1");
    }
}

impl<X: Printable, Y: Printable> Printable for Queen<X, Y> {
    fn print() {
        print!("Queen(");
        X::print();
        print!(", ");
        Y::print();
        print!(")");
    }
}

fn main() {
    type N0 = Z;
    type N1 = S<N0>;
    type N2 = S<N1>;
    type N3 = S<N2>;
    type N4 = S<N3>;
    type N5 = S<N4>;
    type N6 = S<N5>;

    type Config = <() as Solution<N6>>::C;

    Config::print();
}
