# Programovací jazyk Rust jako alternativa k C++

## Osnova

1. Připomenutí jazyka C++
    - Historie, vychází z C, 1980 Bjarne Stroustrup
    - [Pevně nadefinovaný standard](https://www.iso.org/standard/79358.html), podle kterého je vytvořeno více kompilátorů (gcc, clang, MSVC)
    - Osvědčeno v praxi (prohlížeče, game enginy, embedded, ...)
    - Komplexní syntax, spousta nových featur méně intuitivní, protože backwards compatibility (iterátory, range-based for loop)
1. Programovací jazyk Rust
    - Graydon Hoare 2006, Mozilla jako první sponzor, 2020 Rust Foundation
    - [Pevně daná specifikace neexistuje](https://doc.rust-lang.org/reference/index.html), i když je "někdy" v plánu
    - Mladý, vyvíjející se jazyk (pomocí RFC), přesto už hodně známý ([Stack Overflow most loved lang 7 times in a row](https://survey.stackoverflow.co/2022/#overview)); Web tech (Prisma, SWC), aplikace (Figma), AWS, Dropbox, Firefox CSS engine, OS (Linux, Android, ...)
    - striktní typový systém, borrow checking, runtime safety, unsafe bloky
    - komplexnější a mnohem lepší (objektivně) makrový systém původně inspirovaný jazykem Lisp (not anymore), neexistuje podpora pro native variadické funkce.
1. Praktická část
    1. Úvod - co budeme programovat, metodika
        - Rust i Cpp, u obtížnějších počítáme compiler errory a celkový čas, jak těžké to bylo (subjektivní), rychlost spuštění a kompilace, alternativní implementace lepší pro daný jazyk, velikost binárky, implementační rozdíly, funkční rozdíly
    1. jednoduchý konzolový prográmek - větší menší nebo tak něco (error handling)
    1. datová struktura - linked list? (mem management, ukazatele, struktury + impl vs. znalost jazyka)
    1. soutěžní programování - nějaká úloha z Kasiopey/Advent of Code (standardní knihovna, další packages, input parsing)
    1. něco multithreaded/async?
    1. Shrnutí - co rychlejší, grafy/tabulky?
1. DX
    1. kompilátor
        - kompilátor Rustu, cargo, rust-analyzer jsou open source vs. na C++ většinou IDE, protože jejich linter/lang-server
        - více errorů u Rustu může být bad ("ale v Cpp to funguje") - proč se to může hodit, ale je to mnohem těžší
        - jinak fungující importy - nejsou potřeba .h soubory
    1. build systémy
        - cross-platform problémy, dependency management
        - u C++ není first-party solution - cmake, make, premake, ...
        - u Rustu prostě Cargo, ezzz
    1. dependencies
        - crates.io, jeden způsob jak formátovat kód (compiler warning), jednotný způsob vytváření dokumentace, na docs.rs je dokumentace všech externích cratů (MEGA POG)
        - Cpp distribuce často binárek/compile yourself - klonování z GitHubu, problém s linkováním/cross-compiler-compat
        - vcpkg a podobné dependency managery (vyzkoušet prakticky)
    1. testing
        - cargo first-party support vs. CTest, Catch a další third-party řešení
1. Rozdíly v praxi
    - OOP v Rustu - traits vs. classal inheritance (cpp zase nemá interface, jen pure virtual class, což je ale to samé)
    - Rust je známý pro svou bezpečnost, známý rychlostí, ale hlavně oproti jazykům jako Go a TypeScript (proč *asi*) (používá se na náhradu nástrojů napsaných v těchto jazycích)
    - Cpp se používá (hned) když je potřeba rychlost, pro nové projekty ale dnes již často právě Rust/Zig apod., **proč**?? (asi lepší maintainabilita, bezpečnost, méně bugů, lepší DX)
    - Rust má stále problémy, které C++ dávno vyřešilo (proto nutnost se stále rapidně vyvíjet), possible unsoundness, některé věci nedořešené
    - borrow checker občas trollí, i když aplikace je safe (protože full analysis je nemožné) - potřeba ho obcházet (`Rc<T>` a podobné smart pointery) za cenu bezpečnosti.
1. Nízkoúrovňové rozdíly
    - enumy a pattern matching vs. throw/catch, všechno LLVM (tedy ne tak velké rozdíly), jiný name mangling (`#[no_mangle]`, `#[repr(C)]` apod.), UTF-8 vs. ascii a s tím dané problémy

## Other
