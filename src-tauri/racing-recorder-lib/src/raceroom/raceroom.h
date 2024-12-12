#include <windows.h>
#include "r3e.h"

struct RrHandle{
    HANDLE map_handle;
    r3e_shared* map_buffer;
    int success;
};
struct RrTelemetryObj {
    float speed;
    float steerAngle;
    float gas;
    float brake;
    int currentTime;
    int lap;
};

HANDLE map_open();
BOOL map_exists();
struct RrHandle map_init();
void map_close(struct RrHandle handle);

struct RrHandle rr_init();
struct RrTelemetryObj rr_data(struct RrHandle handle);
void rr_close(struct RrHandle handle);