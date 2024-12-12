#include "acc.h"
#include "SharedFileOut.h"

struct AccHandle acc_init(){
	struct SMElement physics = initPhysics();
    struct SMElement graphics = initGraphics();
	struct AccHandle ret;
	ret.graphics = graphics;
	ret.physics = physics;
	return ret;
}
struct SMElement initPhysics(){
    return initSharedMemory("Local\\acpmf_physics");
}
struct SMElement initGraphics(){
    return initSharedMemory("Local\\acpmf_graphics");
}

struct SMElement initSharedMemory(char* sz_name){
    struct SMElement ret;
    ret.hMapFile = CreateFileMapping(INVALID_HANDLE_VALUE, NULL, PAGE_READWRITE, 0, sizeof(struct SPageFilePhysics), sz_name);
	if (!ret.hMapFile)
	{
		//MessageBoxA(GetActiveWindow(), "CreateFileMapping failed", "ACCS", MB_OK);
	}
	ret.mapFileBuffer = (unsigned char*)MapViewOfFile(ret.hMapFile, FILE_MAP_READ, 0, 0, sizeof(struct SPageFilePhysics));
	if (!ret.mapFileBuffer)
	{
		//MessageBoxA(GetActiveWindow(), "MapViewOfFile failed", "ACCS", MB_OK);
	}
    return ret;
}

struct AccTelemetryObj acc_data(struct AccHandle handle){
	struct AccTelemetryObj ret;
	struct SPageFilePhysics* pf = (struct SPageFilePhysics*)handle.physics.mapFileBuffer;
	struct SPageFileGraphic* pG = (struct SPageFileGraphic*)handle.graphics.mapFileBuffer;
	ret.speed = pf->speedKmh;
	ret.steerAngle = pf->steerAngle;
	ret.gas = pf->gas;
	ret.brake = pf->brake;
	ret.currentTime = pG->iCurrentTime;
	ret.lap = pG->completedLaps;
	return ret;
}