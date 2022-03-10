package io.nordnotes;

import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

/**
 * Main class of the nordnotes application.
 */
@Slf4j
@SpringBootApplication
public class NordnotesApplication {
  /**
   * Main entrypoint of nordnotes back-end application.
   *
   * @param args Command line arguments.
   */
  public static void main(String[] args) {
    SpringApplication.run(NordnotesApplication.class, args);
  }
}
