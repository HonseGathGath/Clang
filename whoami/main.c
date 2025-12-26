#include <pwd.h> //getpwuid
#include <stdio.h>
#include <string.h>    //strcmp
#include <sys/types.h> //uid_t
#include <unistd.h>    //getuid

int main(int argc, char *argv[]) {

  int H_flag = 0;
  int S_flag = 0;

  if (argc > 1) {
    if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--home") == 0) {
      H_flag = 1;
    } else if (strcmp(argv[1], "-s") == 0 || strcmp(argv[1], "--shell") == 0) {
      S_flag = 1;
    }
  }

  uid_t user_id = getuid();

  struct passwd *user_info;

  /*
   * struct passwd {
   * char *pw_name,
   * char *pw_passwd,
   * uid_t pw_uid,
   * gid_t pw_gid,
   * char *pw_gecos,
   * char *pw_dir,
   * char*pw_shell
   * }
   */

  user_info = getpwuid(user_id);

  if (H_flag == 1) {
    printf("%s", user_info->pw_dir);
  } else if (S_flag == 1) {
    printf("%s", user_info->pw_shell);
  }

  return 0;
}
