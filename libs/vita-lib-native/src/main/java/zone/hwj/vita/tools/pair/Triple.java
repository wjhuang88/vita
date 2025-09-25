package zone.hwj.vita.tools.pair;

public interface Triple<E1, E2, E3> extends Pair<E1, Pair<E2, E3>> {

    static <E1, E2, E3> Triple<E1, E2, E3> of(E1 first, E2 second, E3 third) {
        return new ImmutableTriple<>(first, Pair.of(second, third));
    }

    default E1 getFirst() {
        return getLeft();
    }

    default E2 getSecond() {
        return getRight().getLeft();
    }

    default E3 getThird() {
        return getRight().getRight();
    }
}
