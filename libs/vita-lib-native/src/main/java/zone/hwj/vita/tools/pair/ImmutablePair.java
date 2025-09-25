package zone.hwj.vita.tools.pair;

public final class ImmutablePair<L, R> implements Pair<L, R> {

    private final L left;
    private final R right;

    ImmutablePair(L left, R right) {
        this.left = left;
        this.right = right;
    }

    @Override
    public L getLeft() {
        return left;
    }

    @Override
    public R getRight() {
        return right;
    }

    @Override
    public L getKey() {
        return getLeft();
    }

    @Override
    public R getValue() {
        return getRight();
    }

    @Override
    public R setValue(R value) {
        throw new UnsupportedOperationException("Cannot set a value on an ImmutablePair");
    }
}
