#include <windows.h>

struct SMElement
{
    HANDLE hMapFile;
    unsigned char* mapFileBuffer;
};

struct AccHandle{
    struct SMElement physics;
    struct SMElement graphics;
};

struct AccTelemetryObj {
    float speed;
    float steerAngle;
    float gas;
    float brake;
    int currentTime;
    int lap;
};


struct AccHandle acc_init();
struct SMElement initPhysics();
struct SMElement initGraphics();
struct SMElement initSharedMemory(char* sz_name);
struct AccTelemetryObj acc_data(struct AccHandle handle);

