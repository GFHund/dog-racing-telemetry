#include "raceroom.h"

HANDLE map_open()
{
    return OpenFileMapping(
        FILE_MAP_READ,
        FALSE,
        TEXT(R3E_SHARED_MEMORY_NAME));
}
BOOL map_exists()
{
    HANDLE handle = map_open();

    if (handle != NULL)
        CloseHandle(handle);
        
    return handle != NULL;
}

struct RrHandle map_init()
{
    HANDLE map_handle = map_open();
    struct RrHandle ret;
    if (map_handle == NULL)
    {
        //wprintf_s(L"Failed to open mapping");
        ret.success = 0;
        return ret;
    }

    r3e_shared* map_buffer = (r3e_shared*)MapViewOfFile(map_handle, FILE_MAP_READ, 0, 0, sizeof(r3e_shared));
    if (map_buffer == NULL)
    {
        //wprintf_s(L"Failed to map buffer");
        ret.success = 0;
        return ret;
    }
    
    ret.map_handle = map_handle;
    ret.map_buffer = map_buffer;
    ret.success = 1;

    return ret;
}

void map_close(struct RrHandle handle)
{
    if (handle.map_buffer) UnmapViewOfFile(handle.map_buffer);
    if (handle.map_handle) CloseHandle(handle.map_handle);
}

struct RrHandle rr_init(){
    return map_init();
}
struct RrTelemetryObj rr_data(struct RrHandle handle){
    struct RrTelemetryObj ret;
    ret.speed = handle.map_buffer->car_speed ;
    ret.gas = handle.map_buffer->throttle_raw;
    ret.brake = handle.map_buffer->brake_raw;
    ret.steerAngle = handle.map_buffer->steer_input_raw;
    ret.currentTime = handle.map_buffer->lap_time_current_self;
    ret.lap = handle.map_buffer->completed_laps;
    return ret;
}
void rr_close(struct RrHandle handle){
    map_close(handle);
}