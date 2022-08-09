package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

package main

import (
	"fmt"
	"os"
	"runtime/debug"
)

const (
	MIN_TERM_WIDTH  = 33
	MIN_TERM_HEIGHT = 15
)

var version = ""

func main() {
	var c color.Color = color.DEFAULT
	var err error

	if args := os.Args[1:]; 1 <= len(args) {
		c, err = parseArgs(args)

		if err != nil {
			fmt.Println(err.Error())
		}
	}

	var t terminal.Terminal
	t.New()

	if t.Width < MIN_TERM_WIDTH || t.Height < MIN_TERM_HEIGHT {
		needSize(t.Width, t.Height)
	}

	go crock(t, c)

	handleSignal(t)
}

