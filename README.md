<h1>:gear: Algorithms on Rust <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/endygamedev/rust_algorithms/Rust?style=flat&logo=github&logoColor=%23939393&labelColor=3a3d3b&color=%2331e26c" hspace="15"></h1>

<h2> :crab: Content </h2> 
<ol>
  <li><b> <a href="#algebra">Algebra</a> </b></li>
  <li><b> <a href="#search">Seach</a> </b></li>
  <li><b> <a href="#sorting">Sorting</a> </b></li>
</ol>


<a id="algebra">
    <h3> :crab: <a href="https://github.com/endygamedev/rust_algorithms/tree/main/src/algorithms/algebra">Algebra</a> </h3>
</a>

<details>
  <summary> 1. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/algebra/gcd.rs">Euclidean algorithm</a> </summary>
      <h6>Short description</h6>
      <p><b>Euclidean algorithm</b> is an efficient method for computing the greatest common divisor (GCD) of two integers, the largest number that divides them both without a remainder.</p>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Euclidean_algorithm">Wiki</a> </h6>
</details>

2. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/algebra/minimal.rs">Minimum of an
   array</a>
3. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/algebra/maximum.rs">Maximum of an array</a>

<a id="search">
    <h3> :crab: <a href="https://github.com/endygamedev/rust_algorithms/tree/main/src/algorithms/searches">Search</a> </h3>
</a>

<details>
  <summary> 1. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/searches/binary_search.rs">Binary search</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Binary search</b> is a search algorithm that finds the position of a target value within a sorted array. Binary search compares the target value to the middle element of the array. If they are not equal, the half in which the target cannot lie is eliminated and the search continues on the remaining half, again taking the middle element to compare to the target value, and repeating this until the target value is found. If the search ends with the remaining half being empty, the target is not in the array.</p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(log <i>n</i>) </li>
        <li> Worst-case: <i>O</i>(log <i>n</i>) </li>
        <li> Best-case: <i>O</i>(1) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Binary_search_algorithm">Wiki</a> </h6>
    </p>
</details>

<details>
  <summary> 2. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/searches/linear_search.rs">Linear search</a> </summary>
    <p>
      <h6>Short description</h6>
      <p>A <b>linear search</b> sequentially checks each element of the list until it finds an element that matches the target value. If the algorithm reaches the end of the list, the search terminates unsuccessfully.</p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(<i>n</i>/2) </li>
        <li> Worst-case: <i>O</i>(<i>n</i>) </li>
        <li> Best-case: <i>O</i>(1) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Linear_search">Wiki</a> </h6>
    </p>
</details>


<a id="sorting">
    <h3> :crab: <a href="https://github.com/endygamedev/rust_algorithms/tree/main/src/algorithms/sorts">Sorting</a> </h3>
</a>

<details>
  <summary> 1. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/sorts/bubble_sort.rs">Bubble sort</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Bubble sort</b> is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted. The algorithm, which is a comparison sort, is named for the way smaller or larger elements "bubble" to the top of the list.</p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Worst-case: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Best-case: <i>O</i>(1) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Bubble_sort">Wiki</a> </h6>
    </p>
</details>

<details>
  <summary> 2. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/sorts/insertion_sort.rs">Insertion sort</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Insertion sort</b> is a simple sorting algorithm that builds the final sorted array (or list) one item at a time. Sorting is typically done in-place, by iterating up the array, growing the sorted list behind it. At each array-position, it checks the value there against the largest value in the sorted list (which happens to be next to it, in the previous array-position checked). If larger, it leaves the element in place and moves to the next. If smaller, it finds the correct position within the sorted list, shifts all the larger values up to make a space, and inserts into that correct position.</p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Worst-case: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Best-case: <i>O</i>(1) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Insertion_sort">Wiki</a> </h6>
    </p>
</details>

<details>
  <summary> 3. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/sorts/selection_sort.rs">Selection sort</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Selection sort</b> is an in-place comparison sorting algorithm. The algorithm divides the input list into two parts: a sorted sublist of items which is built up from left to right at the front (left) of the list and a sublist of the remaining unsorted items that occupy the rest of the list. Initially, the sorted sublist is empty and the unsorted sublist is the entire input list. The algorithm proceeds by finding the smallest (or largest, depending on sorting order) element in the unsorted sublist, exchanging (swapping) it with the leftmost unsorted element (putting it in sorted order), and moving the sublist boundaries one element to the right.</p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Worst-case: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Best-case: <i>O</i>(1) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Selection_sort">Wiki</a> </h6>
    </p>
</details>

<details>
  <summary> 4. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/sorts/quick_sort.rs">Quicksort</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Quicksort</b> is a divide and conquer algorithm. It first divides the input array into two smaller sub-arrays: the low elements and the high elements. It then recursively sorts the sub-arrays. <ins>The steps for in-place Quicksort are:</ins>
      <ol>
        <li> Pick an element, called a pivot, from the array. </li>
        <li> Partitioning: reorder the array so that all elements with values less than the pivot come before the pivot, while all elements with values greater than the pivot come after it (equal values can go either way). After this partitioning, the pivot is in its final position. This is called the partition operation. </li>
        <li> Recursively apply the above steps to the sub-array of elements with smaller values and separately to the sub-array of elements with greater values. </li>
      </ol>
    </p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(<i>n</i> log <i>n</i>) </li>
        <li> Worst-case: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Best-case: <i>O</i>(<i>n</i> log <i>n</i>) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Quicksort">Wiki</a> </h6>
    </p>
</details>

<details>
  <summary> 5. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/sorts/insertion_sort.rs">Merge sort</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Merge sort</b> is a divide and conquer algorithm. <ins>Conceptually, a merge sort works as follows:</ins>
        <ol>
        <li> Divide the unsorted list into n sublists, each containing one element (a list of one element is considered sorted). </li>
        <li> Repeatedly <a href="https://en.wikipedia.org/wiki/Merge_algorithm">merge</a> sublists to produce new sorted sublists until there is only one sublist remaining. This will be the sorted list. </li>
      </ol>
      </p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(<i>n</i> log <i>n</i>) </li>
        <li> Worst-case: <i>O</i>(<i>n</i> log <i>n</i>) </li>
        <li> Best-case: <i>O</i>(<i>n</i> log <i>n</i>) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Merge_sort">Wiki</a> </h6>
    </p>
</details>

<details>
  <summary> 6. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/sorts/shell_sort.rs">Shell sort</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Shell sort</b> is an in-place comparison sort. It can be seen as either a generalization of sorting by exchange (bubble sort) or sorting by insertion (insertion sort). The method starts by sorting pairs of elements far apart from each other, then progressively reducing the gap between elements to be compared. By starting with far apart elements, it can move some out-of-place elements into position faster than a simple nearest neighbor exchange.</p>
      <h6> Performance </h6>
      <ul>
        <li> Average: depends on gap sequence </li>
        <li> Worst-case: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Best-case: <i>O</i>(<i>n</i> log <i>n</i>) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Shellsort">Wiki</a> </h6>
    </p>
</details>

<details>
  <summary> 7. <a href="https://github.com/endygamedev/rust_algorithms/blob/main/src/algorithms/sorts/cocktail_sort.rs">Cocktail shaker sort</a> </summary>
    <p>
      <h6>Short description</h6>
      <p><b>Cocktail shaker sort</b> is a slight variation of bubble sort. It differs in that instead of repeatedly passing through the list from bottom to top, it passes alternately from bottom to top and then from top to bottom. It can achieve slightly better performance than a standard bubble sort. The reason for this is that bubble sort only passes through the list in one direction and therefore can only move items backward one step each iteration.</p>
      <h6> Performance </h6>
      <ul>
        <li> Average: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Worst-case: <i>O</i>(<i>n</i><sup>2</sup>) </li>
        <li> Best-case: <i>O</i>(<i>n</i>) </li>
      </ul>
      <h6> View full on <a href="https://en.wikipedia.org/wiki/Cocktail_shaker_sort">Wiki</a> </h6>
    </p>
</details>


<br>
<p align="center">
    :arrow_up: <a href="#"><b>Back to top</b></a> :arrow_up:
</p>

<br>
<p align="center">
  <sub> | <a href="https://endygamedev.github.io"> 👨‍💻 endygamdev </a> | </sub>
</p>
