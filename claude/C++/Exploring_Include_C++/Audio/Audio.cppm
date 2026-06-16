export module Audio;

export class Audio
{
    private: static int Clamp(int value, int minimum, int maximum)
    {
        if (value < minimum)
        {
            return minimum;
        }

        if (value > maximum)
        {
            return maximum;
        }

        return value;
    }

    public: static int CalculateVolume(int baseVolume, int multiplier)
    {
        int volume = baseVolume * multiplier;

        return Clamp(volume, 0, 100);
    }

    public: static int CalculatePan(int leftChannel, int rightChannel)
    {
        int difference = rightChannel - leftChannel;

        return Clamp(difference, -100, 100);
    }
};
