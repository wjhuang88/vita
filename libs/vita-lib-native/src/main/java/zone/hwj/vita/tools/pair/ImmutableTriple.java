package zone.hwj.vita.tools.pair;

public final class ImmutableTriple<E1, E2, E3> implements Triple<E1, E2, E3> {

    private final E1 first;

    private final Pair<E2, E3> nextPair;

    ImmutableTriple(E1 first, Pair<E2, E3> nextPair) {
        this.first = first;
        this.nextPair = nextPair;
    }

    @Override
    public E1 getLeft() {
        return first;
    }

    @Override
    public Pair<E2, E3> getRight() {
        return nextPair;
    }

    @Override
    public E1 getKey() {
        return first;
    }

    @Override
    public Pair<E2, E3> getValue() {
        return nextPair;
    }

    @Override
    public Pair<E2, E3> setValue(Pair<E2, E3> value) {
        throw new UnsupportedOperationException("Cannot set a value on an ImmutableTriple");
    }
}
