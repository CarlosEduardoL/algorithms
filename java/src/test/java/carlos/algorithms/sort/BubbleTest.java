package carlos.algorithms.sort;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class BubbleTest extends SortTests {

    @Override
    void sort(List<Integer> elements) {
        Bubble.sort(elements);
    }
}