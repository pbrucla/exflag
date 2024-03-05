# =====================================================
#          exflag1 (binary oracle), py edition
#                     by Arc'blroth
# =====================================================

from typing import Callable, List


def parse_glob(glob: str) -> List[int]:
    """
    "parse" the charset "glob"
    """
    lexed_glob = []
    parse_pos = 0
    eof_pos = len(glob)
    while parse_pos < len(glob):
        if glob[parse_pos] == "\\":
            if parse_pos + 1 == eof_pos:
                raise SyntaxError('truncated escape in charset "glob"')
            match glob[parse_pos + 1]:
                case "\\":
                    lexed_glob.append("\\")
                case "-":
                    lexed_glob.append("-")
                case _:
                    raise SyntaxError('invalid escape in charset "glob"')
            parse_pos += 2
        elif glob[parse_pos] == "-":
            lexed_glob.append(None)
            parse_pos += 1
        else:
            lexed_glob.append(glob[parse_pos])
            parse_pos += 1
    parsed_glob = []
    parse_pos = 0
    eof_pos = len(lexed_glob)
    while parse_pos < len(lexed_glob):
        if lexed_glob[parse_pos] is None:
            raise SyntaxError('invalid range in charset "glob"')
        if parse_pos + 1 != eof_pos and lexed_glob[parse_pos + 1] is None:
            if parse_pos + 2 == eof_pos:
                raise SyntaxError('invalid range in charset "glob"')
            parsed_glob += list(range(ord(lexed_glob[parse_pos]), ord(lexed_glob[parse_pos + 2]) + 1))
            parse_pos += 3
        else:
            parsed_glob.append(ord(lexed_glob[parse_pos]))
            parse_pos += 1
    return parsed_glob


def extract_flag(
    oracle: Callable[[str], bool],
    prefix: str = "",
    charset: List[int] = "_0-9A-Za-z}",
    stop: Callable[[str], bool] = lambda x: x.endswith("}"),
) -> str:
    """
    Exfiltrates a flag, character by character, given a binary oracle and a charset.

    :param oracle: a binary oracle that checks if the given prefix of the flag is valid
    :param prefix: known flag prefix to start with
    :param charset: a charset, in "glob"-like format, to try characters from
    :param stop: stopping oracle (defaults to flag having `}`)
    :raises Exception: if a flag cannot be found using the given charset
    """
    charset = parse_glob(charset)
    if not charset:
        raise ValueError("charset is empty")
    flag = prefix
    while not stop(flag):
        flag_len = len(flag)
        for char in charset:
            print(f'\x1b[90mtrying "{flag}{chr(char)}"\x1b[0m')
            if oracle(flag + chr(char)):
                flag += chr(char)
                print(f'\x1b[33mfound "{flag}"\x1b[0m')
                break
        if len(flag) == flag_len:
            raise Exception("failed to solve next flag character, aborting")
