#define MAX_WAVES 16

typedef unsigned int usize;

typedef struct {
  float amp;
  float freq;
  float phase;
} Wave;

static float AMPS[MAX_WAVES];
static float FREQS[MAX_WAVES];
static float PHASES[MAX_WAVES];

static float wrap_angle(float angle) {
  const float pi = 3.14159265358979323846f;
  const float two_pi = 6.28318530717958647692f;
  int turns = (int)(angle / two_pi);

  angle -= (float)turns * two_pi;

  if (angle > pi) {
    angle -= two_pi;
  } else if (angle < -pi) {
    angle += two_pi;
  }

  return angle;
}

static float cos_poly(float angle) {
  float x2 = angle * angle;
  return 1.0f - x2 * (0.5f - x2 * (0.0416666679f - x2 * (0.0013888890f - x2 * 0.0000248016f)));
}

static float sin_poly(float angle) {
  float x2 = angle * angle;
  return angle * (1.0f - x2 * (0.1666666716f - x2 * (0.0083333310f - x2 * 0.0001984090f)));
}

static float cos_approx(float angle) {
  const float half_pi = 1.57079632679489661923f;
  const float pi = 3.14159265358979323846f;

  angle = wrap_angle(angle);

  if (angle > half_pi) {
    return -cos_poly(pi - angle);
  }

  if (angle < -half_pi) {
    return -cos_poly(pi + angle);
  }

  return cos_poly(angle);
}

static float sin_approx(float angle) {
  const float half_pi = 1.57079632679489661923f;
  const float pi = 3.14159265358979323846f;

  angle = wrap_angle(angle);

  if (angle > half_pi) {
    return sin_poly(pi - angle);
  }

  if (angle < -half_pi) {
    return -sin_poly(pi + angle);
  }

  return sin_poly(angle);
}

static float combine_wave(Wave wave, float t, float step) {
  float initial_angle = wave.freq * t + wave.phase;
  float angle_delta = wave.freq * step;
  float cos_delta = cos_approx(angle_delta);
  float sin_delta = sin_approx(angle_delta);
  float cos_angle = cos_approx(initial_angle);
  float sin_angle = sin_approx(initial_angle);

  float next_cos = cos_angle * cos_delta - sin_angle * sin_delta;
  float next_sin = sin_angle * cos_delta + cos_angle * sin_delta;
  (void)next_cos;
  (void)next_sin;

  return wave.amp * cos_angle;
}

__attribute__((export_name("max_waves")))
usize max_waves(void) {
  return MAX_WAVES;
}

__attribute__((export_name("amps_ptr")))
float *amps_ptr(void) {
  return AMPS;
}

__attribute__((export_name("freqs_ptr")))
float *freqs_ptr(void) {
  return FREQS;
}

__attribute__((export_name("phases_ptr")))
float *phases_ptr(void) {
  return PHASES;
}

__attribute__((export_name("combine")))
float combine(usize count, float t, float step) {
  usize capped_count = count < MAX_WAVES ? count : MAX_WAVES;
  float sum = 0.0f;

  for (usize index = 0; index < capped_count; index += 1) {
    Wave wave = {
      AMPS[index],
      FREQS[index],
      PHASES[index],
    };

    sum += combine_wave(wave, t, step);
  }

  return sum;
}
