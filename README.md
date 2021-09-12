# Variables
```
// Immutable variable
val x = 10

// Mutable variable
var y = 3
y = 5
```

# Functions
```
def greet(name: String)
	print("Hello, $name!")
end

greet("John") // Prints "Hello, John!"

def double(x: int): int
	x * 2
end

print(double(5)) // Prints "10"
```

# Classes
```
class Rectangle (var width: int, var height: int)

// Class method
def (Rectangle) new(width: int, height: int): Rectangle
	Rectangle { width: 5, height: 5}
end

val rect = Rectangle.new(5, 5)
```

# Traits
```
trait Show
	fun show(f: A): String
end
```

# Algebraic datatypes
```
enum Color
	RED,
	BLUE,
	GREEN,
end
```

## Examples
```
trait Shape
	def name(): String
	def area(): int
end

class Rectangle
	val width: int
	val height: int
end

class Circle
	radius: int
end

def printShapeInfo<T>()
```