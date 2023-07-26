class Random {
  static number(min, max) {
    let r = Math.random();
    let m = max - min;
    let rest = r * m + min;
    return rest;
  }
  static int(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min)) + min;
  }
}

class Canvas {
  constructor() {
    this.canvas = document.getElementById("canvas");
    if (!this.canvas) {
      this.canvas = document.createElement("canvas");
      document.body.appendChild(this.canvas);
      this.canvas.width = window.innerWidth - 30;
      this.canvas.height = window.innerHeight - 30;
      this.canvas.style.border = "1px solid black";
    }
    this.ctx = this.canvas.getContext("2d");

    this.canvas.addEventListener("click", (e) => {
      this.mouseClick(e);
    });
  }
  clear(color) {
    if (!color) {
      this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    } else {
      this.ctx.fillStyle = color;
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    }
  }
  rectangle(r, color, weight) {
    this.ctx.lineWidth = weight || 1;
    this.ctx.strokeStyle = color;
    this.ctx.strokeRect(r.x, r.y, r.w, r.h);
  }
  point(p, weigth, color) {
    this.ctx.fillStyle = color;
    let w = weigth || 1;
    let x = p.x;
    let y = p.y;
    if (w > 1) {
      let n = w / 2;
      x -= n;
      x -= n;
    }
    this.ctx.fillRect(x, y, w, w);
  }
  line(x1, y1, x2, y2, color) {
    this.ctx.strokeStyle = color;
    this.ctx.beginPath();
    this.ctx.moveTo(x1, y1);
    this.ctx.lineTo(x2, y2);
    this.ctx.stroke();
  }
  mouseClick(e) {
    this.mousePos = new Point(e.layerX, e.layerY);
    this.onMouseClick(this.mousePos);
  }
}

class Rectangle {
  constructor(x, y, w, h) {
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
  }
  contains(p) {
    return (
      p.x >= this.x &&
      p.x <= this.x + this.w &&
      p.y >= this.y &&
      p.y <= this.y + this.h
    );
  }
  intersects(r) {
    return Rectangle.intersects(this, r);
  }
  static intersects(a, b) {
    let centerA = new Point(a.x + a.w / 2, a.y + a.h / 2);
    let centerB = new Point(b.x + b.w / 2, b.y + b.h / 2);
    let diffX = Math.abs(centerA.x - centerB.x);
    let diffY = Math.abs(centerA.y - centerB.y);

    let dx = a.w / 2 + b.w / 2;
    let dy = a.h / 2 + b.h / 2;
    return diffX <= dx && diffY <= dy;
  }
}

class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
}
class QuadTree {
  constructor(capacity, boundary) {
    this.capacity = capacity;
    this.boundary = boundary;
    this.points = [];
  }
  add(p) {
    if (!this.boundary.contains(p)) {
      return false;
    }
    if (this.points.length < this.capacity) {
      this.points.push(p);
      return true;
    }
    if (!this.topleft) this.divide();
    let result =
      this.topleft.add(p) ||
      this.topright.add(p) ||
      this.bottomleft.add(p) ||
      this.bottomright.add(p);
    return result;
  }
  divide() {
    let width = this.boundary.w / 2;
    let height = this.boundary.h / 2;
    this.topleft = new QuadTree(
      this.capacity,
      new Rectangle(this.boundary.x, this.boundary.y, width, height)
    );
    this.topright = new QuadTree(
      this.capacity,
      new Rectangle(this.boundary.x + width, this.boundary.y, width, height)
    );
    this.bottomleft = new QuadTree(
      this.capacity,
      new Rectangle(this.boundary.x, this.boundary.y + height, width, height)
    );
    this.bottomright = new QuadTree(
      this.capacity,
      new Rectangle(
        this.boundary.x + width,
        this.boundary.y + height,
        width,
        height
      )
    );
  }
  draw(canvas) {
    canvas.rectangle(this.boundary, "white");
    this.points.forEach((p) => {
      canvas.point(p, 4, "white");
    });
    if (this.topleft) {
      this.topleft.draw(canvas);
      this.topright.draw(canvas);
      this.bottomleft.draw(canvas);
      this.bottomright.draw(canvas);
    }
  }
  query(range) {
    let found = [];
    if (this.boundary.intersects(range)) {
      this.points.forEach((p) => {
        if (range.contains(p)) {
          found.push(p);
        }
      });
      if (this.topleft) {
        found = found.concat(this.topleft.query(range));
        found = found.concat(this.topright.query(range));
        found = found.concat(this.bottomleft.query(range));
        found = found.concat(this.bottomright.query(range));
      }
    }
    return found;
  }
}

let ca = new Canvas();
let rect = new Rectangle(0, 0, ca.canvas.width, ca.canvas.height);
let qt;
let distance = 20;

let points = [];
for (let x = 1; x < 2000; x++) {
  let x = Random.int(0, ca.canvas.width);
  let y = Random.int(0, ca.canvas.height);
  let p = new Point(x, y);
  points.push(p);
}

ca.onMouseClick = (pos) => {
  let r = new Rectangle(pos.x, pos.y, 200, 200);
  ca.clear("black");
  qt.draw(ca);
  ca.rectangle(r, "green", 4);
  let qPoints = qt.query(r);
  qPoints.forEach((p) => {
    if (r.contains(p)) {
      ca.point(p, 5, "yellow");
    }
  });
};

const update = () => {
  qt = new QuadTree(4, rect);

  points.forEach((p) => {
    p.x += Random.number(-2, 2);
    p.y += Random.number(-2, 2);
    qt.add(p);
  });
};
const render = () => {
  ca.clear("black");
  qt.draw(ca);

  linesQt();
};
function linesQt() {
  points.forEach((e) => {
    let r = new Rectangle(
      e.x - distance,
      e.y - distance,
      distance * 2,
      distance * 2
    );
    let qPoints = qt.query(r);
    qPoints.forEach((qp) => {
      if (qp !== e) {
        checkHypot(e, qp);
      }
    });
  });
}
function lines() {
  for (let i = 0; i < points.length - 1; i++) {
    let a = points[i];
    for (let c = 0; c < points.length; c++) {
      let b = points[c];
      checkHypot(a, b);
    }
  }
}

function checkHypot(a, b) {
  let x = a.x - b.x;
  let y = a.y - b.y;
  let h = Math.hypot(x, y);
  if (h <= distance) {
    ca.line(a.x, a.y, b.x, b.y, "blue");
  }
}
const execute = () => {
  update();
  render();
  requestAnimationFrame(execute);
};

requestAnimationFrame(execute);
