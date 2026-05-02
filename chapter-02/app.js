// Exercise Data
const EXERCISES = [
  // Concepts
  {
    id: 'compile',
    type: 'concept',
    title: 'Compile & Run',
    spec: 'Compile a program with rustc that prints output. Migrate it to a cargo project and run with cargo run.',
    mapPos: { x: 80, y: 150 }
  },
  {
    id: 'variables',
    type: 'concept',
    title: 'Variables & Types',
    spec: 'Declare variables with inferred type, explicit i32, and inline type syntax. Create a function that adds two integers.',
    mapPos: { x: 180, y: 100 }
  },
  {
    id: 'numbers',
    type: 'concept',
    title: 'Numbers & Arithmetic',
    spec: 'Create floating-point literals using different syntax styles. Print with format specifiers. Add large integers using underscore notation.',
    mapPos: { x: 280, y: 80 }
  },
  {
    id: 'number-bases',
    type: 'concept',
    title: 'Binary, Octal, Hex',
    spec: 'Declare integers using binary, octal, and hexadecimal notation. Print each in its original base using format specifiers.',
    mapPos: { x: 380, y: 100 }
  },
  {
    id: 'comparing',
    type: 'concept',
    title: 'Comparing Numbers',
    spec: 'Compare integers of different types directly and observe the type error. Fix using as casting, then try try_into().',
    mapPos: { x: 480, y: 150 }
  },
  {
    id: 'match-basic',
    type: 'concept',
    title: 'Match Expressions',
    spec: 'Pattern-match on an integer with range patterns to assign letter grades. Use a catch-all pattern for invalid values.',
    mapPos: { x: 580, y: 100 }
  },
  {
    id: 'match-advanced',
    type: 'concept',
    title: 'Match with OR',
    spec: 'Search through a collection for multiple values using OR patterns in match. Print when matches are found.',
    mapPos: { x: 680, y: 150 }
  },
  {
    id: 'for-loops',
    type: 'concept',
    title: 'For Loops',
    spec: 'Iterate over a collection three ways: taking ownership, borrowing, and borrowing mutably. Observe what compiles.',
    mapPos: { x: 780, y: 100 }
  },
  {
    id: 'while-loop',
    type: 'concept',
    title: 'While Loop',
    spec: 'Measure how many iterations you can complete in one second using system time.',
    mapPos: { x: 880, y: 150 }
  },
  {
    id: 'loop-break',
    type: 'concept',
    title: 'Loop & Break',
    spec: 'Use an infinite loop with break to find a number meeting specific criteria. Return the value via break.',
    mapPos: { x: 980, y: 100 }
  },
  {
    id: 'if-else',
    type: 'concept',
    title: 'If as Expression',
    spec: 'Use if-else as an expression to assign values. Implement a helper predicate function.',
    mapPos: { x: 1080, y: 150 }
  },
  {
    id: 'references',
    type: 'concept',
    title: 'References',
    spec: 'Store a value and create a reference to it. Use both the original and dereferenced reference in arithmetic.',
    mapPos: { x: 1100, y: 250 }
  },
  {
    id: 'strings',
    type: 'concept',
    title: 'Strings & str',
    spec: 'Create string literals and convert between &str and String. Append to strings and check their length.',
    mapPos: { x: 950, y: 300 }
  },
  {
    id: 'arrays-slices',
    type: 'concept',
    title: 'Arrays & Slices',
    spec: 'Create fixed-size arrays of different types. Write a generic function accepting slice references.',
    mapPos: { x: 800, y: 350 }
  },
  {
    id: 'vectors',
    type: 'concept',
    title: 'Vectors',
    spec: 'Collect values in a growable Vec. Perform in-place operations like sorting and filtering. Print with indices.',
    mapPos: { x: 650, y: 300 }
  },
  {
    id: 'generics',
    type: 'concept',
    title: 'Generic Functions',
    spec: 'Write a generic function with trait bounds. Call it with different types to verify it works.',
    mapPos: { x: 500, y: 350 }
  },
  {
    id: 'lifetimes',
    type: 'concept',
    title: 'Lifetime Annotations',
    spec: 'Annotate explicit lifetimes in a function signature. Understand why multiple lifetimes are needed.',
    mapPos: { x: 350, y: 300 }
  },
  {
    id: 'complex-numbers',
    type: 'concept',
    title: 'Third-Party Crates',
    spec: 'Use a third-party crate to work with complex numbers. Perform arithmetic and format output.',
    mapPos: { x: 200, y: 350 }
  },

  // Projects
  {
    id: 'grep-v1',
    type: 'project',
    title: 'grep-lite v1',
    spec: 'Build a simple pattern matcher on hardcoded text using string methods. Print matching lines.',
    mapPos: { x: 150, y: 500 }
  },
  {
    id: 'grep-v2',
    type: 'project',
    title: 'grep-lite v2',
    spec: 'Add line numbering to output using enumeration.',
    mapPos: { x: 350, y: 500 }
  },
  {
    id: 'grep-v3',
    type: 'project',
    title: 'grep-lite v3',
    spec: 'Extend to capture surrounding context lines for each match.',
    mapPos: { x: 550, y: 500 }
  },
  {
    id: 'grep-v4',
    type: 'project',
    title: 'grep-lite v4',
    spec: 'Accept a pattern argument and use regex for flexible matching.',
    mapPos: { x: 750, y: 500 }
  },
  {
    id: 'grep-v5',
    type: 'project',
    title: 'grep-lite v5',
    spec: 'Read from either a file or stdin based on arguments. Abstract the core logic into a generic function.',
    mapPos: { x: 950, y: 500 }
  },
  {
    id: 'mandelbrot',
    type: 'project',
    title: 'Mandelbrot Set',
    spec: 'Use complex numbers to render the Mandelbrot set. Output using ASCII characters based on escape time.',
    mapPos: { x: 1100, y: 500 }
  }
];

// Utility: Get completion status
function isComplete(id) {
  return localStorage.getItem(`ch2:${id}`) === 'true';
}

// Utility: Toggle completion
function toggleComplete(id) {
  const key = `ch2:${id}`;
  const current = localStorage.getItem(key) === 'true';
  localStorage.setItem(key, current ? 'false' : 'true');
  render();
}

// Render Progress Counter
function renderProgress() {
  const completed = EXERCISES.filter(e => isComplete(e.id)).length;
  document.getElementById('progress-count').textContent = completed;
  document.getElementById('progress-total').textContent = EXERCISES.length;
}

// Render Concept Map
function renderMap() {
  const svg = document.getElementById('concept-map');
  const edgesGroup = svg.querySelector('#map-edges');
  const nodesGroup = svg.querySelector('#map-nodes');

  edgesGroup.innerHTML = '';
  nodesGroup.innerHTML = '';

  // Add arrow marker definition
  const defs = svg.querySelector('defs') || document.createElementNS('http://www.w3.org/2000/svg', 'defs');
  if (!svg.querySelector('defs')) {
    svg.appendChild(defs);
  }
  defs.innerHTML = `
    <marker id="arrowhead" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
      <polygon points="0 0, 10 3, 0 6" fill="#d1d5db" />
    </marker>
  `;

  // Draw edges (concept progression flow)
  const edges = [
    ['compile', 'variables'],
    ['variables', 'numbers'],
    ['numbers', 'number-bases'],
    ['number-bases', 'comparing'],
    ['comparing', 'match-basic'],
    ['match-basic', 'match-advanced'],
    ['match-advanced', 'for-loops'],
    ['for-loops', 'while-loop'],
    ['while-loop', 'loop-break'],
    ['loop-break', 'if-else'],
    ['if-else', 'references'],
    ['references', 'strings'],
    ['strings', 'arrays-slices'],
    ['arrays-slices', 'vectors'],
    ['vectors', 'generics'],
    ['generics', 'lifetimes'],
    ['lifetimes', 'complex-numbers'],
  ];

  edges.forEach(([from, to]) => {
    const fromEx = EXERCISES.find(e => e.id === from);
    const toEx = EXERCISES.find(e => e.id === to);
    if (fromEx && toEx) {
      const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
      line.setAttribute('x1', fromEx.mapPos.x);
      line.setAttribute('y1', fromEx.mapPos.y);
      line.setAttribute('x2', toEx.mapPos.x);
      line.setAttribute('y2', toEx.mapPos.y);
      line.setAttribute('class', 'edge');
      edgesGroup.appendChild(line);
    }
  });

  // Draw nodes
  const nodeRadius = 35;
  EXERCISES.forEach(ex => {
    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
    circle.setAttribute('cx', ex.mapPos.x);
    circle.setAttribute('cy', ex.mapPos.y);
    circle.setAttribute('r', nodeRadius);
    circle.setAttribute('class', isComplete(ex.id) ? 'done' : 'todo');
    circle.addEventListener('click', () => {
      const card = document.querySelector(`[data-exercise-id="${ex.id}"]`);
      if (card) {
        card.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        card.classList.add('highlight');
        setTimeout(() => card.classList.remove('highlight'), 1000);
      }
    });
    nodesGroup.appendChild(circle);

    // Label
    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('x', ex.mapPos.x);
    text.setAttribute('y', ex.mapPos.y + nodeRadius + 18);
    text.setAttribute('class', `node-label ${isComplete(ex.id) ? 'done' : ''}`);
    text.setAttribute('text-anchor', 'middle');
    text.textContent = ex.title.substring(0, 12);
    nodesGroup.appendChild(text);
  });
}

// Render Checklist
function renderChecklist() {
  const container = document.getElementById('exercises-list');
  container.innerHTML = '';

  EXERCISES.forEach((ex, index) => {
    const card = document.createElement('div');
    card.className = `exercise-card ${isComplete(ex.id) ? 'done' : ''}`;
    card.dataset.exerciseId = ex.id;

    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.checked = isComplete(ex.id);
    checkbox.addEventListener('change', () => toggleComplete(ex.id));

    const content = document.createElement('div');
    content.className = 'exercise-content';

    const titleContainer = document.createElement('div');
    titleContainer.style.display = 'flex';
    titleContainer.style.alignItems = 'center';
    titleContainer.style.gap = '0.5rem';

    const number = document.createElement('span');
    number.style.color = '#9ca3af';
    number.style.fontWeight = '500';
    number.textContent = `${index + 1}.`;

    const title = document.createElement('div');
    title.className = 'exercise-title';
    title.style.margin = '0';
    title.textContent = ex.title;

    const badge = document.createElement('span');
    badge.style.fontSize = '0.75rem';
    badge.style.fontWeight = '600';
    badge.style.padding = '0.25rem 0.5rem';
    badge.style.borderRadius = '3px';
    badge.style.marginLeft = 'auto';
    badge.style.color = '#666';
    badge.textContent = ex.type === 'concept' ? 'CONCEPT' : 'PROJECT';
    badge.style.backgroundColor = ex.type === 'concept' ? '#dbeafe' : '#fef3c7';

    titleContainer.appendChild(number);
    titleContainer.appendChild(title);
    titleContainer.appendChild(badge);

    const spec = document.createElement('div');
    spec.className = 'exercise-spec';
    spec.textContent = ex.spec;

    content.appendChild(titleContainer);
    content.appendChild(spec);
    card.appendChild(checkbox);
    card.appendChild(content);

    // Click card to toggle
    card.addEventListener('click', e => {
      if (e.target !== checkbox) {
        checkbox.click();
      }
    });

    container.appendChild(card);
  });
}

// Main render function
function render() {
  renderProgress();
  renderMap();
  renderChecklist();
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', render);
