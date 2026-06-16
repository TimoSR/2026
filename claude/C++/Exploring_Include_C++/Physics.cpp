#ifndef UNITY_BUILD
#include "Math.cpp"
#endif

class Physics 
{
    public: static float CalculateForce(float mass, float acceleration) 
    {
        return Math::Add(mass, acceleration); // Assumes Math exists
    }

    private: static float add(int num1, int num2) 
    {
        return Math::Add(num1, num2);
    }
};