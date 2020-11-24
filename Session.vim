let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd c:/Users/Zack/source/nums
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +1 ~/source/nums/Cargo.toml
badd +82 ~/source/nums/src/main.rs
badd +13 src/ast.rs
badd +28 ~/source/nums/src/scan.rs
badd +50 src/parse.rs
argglobal
%argdel
edit ~/source/nums/src/main.rs
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd _ | wincmd |
split
1wincmd k
wincmd w
wincmd w
set nosplitbelow
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 27 + 31) / 62)
exe 'vert 1resize ' . ((&columns * 109 + 124) / 249)
exe '2resize ' . ((&lines * 32 + 31) / 62)
exe 'vert 2resize ' . ((&columns * 109 + 124) / 249)
exe 'vert 3resize ' . ((&columns * 139 + 124) / 249)
argglobal
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 10 - ((9 * winheight(0) + 13) / 27)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
10
normal! 029|
wincmd w
argglobal
if bufexists("~/source/nums/src/scan.rs") | buffer ~/source/nums/src/scan.rs | else | edit ~/source/nums/src/scan.rs | endif
if &buftype ==# 'terminal'
  silent file ~/source/nums/src/scan.rs
endif
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 41 - ((10 * winheight(0) + 16) / 32)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
41
normal! 032|
wincmd w
argglobal
if bufexists("src/parse.rs") | buffer src/parse.rs | else | edit src/parse.rs | endif
if &buftype ==# 'terminal'
  silent file src/parse.rs
endif
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 39 - ((38 * winheight(0) + 30) / 60)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
39
normal! 025|
wincmd w
exe '1resize ' . ((&lines * 27 + 31) / 62)
exe 'vert 1resize ' . ((&columns * 109 + 124) / 249)
exe '2resize ' . ((&lines * 32 + 31) / 62)
exe 'vert 2resize ' . ((&columns * 109 + 124) / 249)
exe 'vert 3resize ' . ((&columns * 139 + 124) / 249)
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOFc
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
